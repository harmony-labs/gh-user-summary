use chrono::{DateTime, Utc, Datelike, Duration, NaiveDate, TimeZone};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use reqwest::blocking::Client;
use std::env;
use std::error::Error;
use std::collections::HashMap;

mod logging;

#[derive(Deserialize, Debug, Clone)]
struct GitHubEvent {
    created_at: String,
    #[serde(rename = "type")]
    event_type: String,
    repo: Repository,
    payload: Value,
}

#[derive(Deserialize, Debug, Clone)]
struct Repository {
    name: String,
}

#[derive(Deserialize, Debug)]
struct CommitDetail {
    sha: String,
    commit: CommitInfo,
}

#[derive(Deserialize, Debug)]
struct CommitInfo {
    message: String,
}

#[derive(Deserialize, Debug)]
struct PullRequestDetail {
    number: i32,
    title: String,
    body: Option<String>,
    state: String,
    merged: bool,
}

fn main() -> Result<(), Box<dyn Error>> {
    logging::init_logging().expect("Failed to initialize logging");
    log::debug!("Starting gh-user-summary...");

    let args: Vec<String> = env::args().collect();
    log::debug!("Command line args: {:?}", args);
    if args.len() != 3 {
        log::error!("Usage: {} <github-username> <YYYY-MM>", args[0]);
        return Ok(());
    }

    let username = &args[1];
    let month_input = &args[2];
    log::info!("Processing for username: {}, month: {}", username, month_input);

    let token = env::var("GITHUB_TOKEN").unwrap_or_default();
    if token.is_empty() {
        log::warn!("No GITHUB_TOKEN found in environment");
    } else {
        log::debug!("GITHUB_TOKEN found (length: {})", token.len());
    }

    let naive_date = NaiveDate::parse_from_str(&format!("{}-01", month_input), "%Y-%m-%d")?;
    log::debug!("Naive date parsed: {}", naive_date);
    let start_date = Utc.from_utc_datetime(&naive_date.and_hms_opt(0, 0, 0).unwrap());
    let next_month = if naive_date.month() == 12 {
        NaiveDate::from_ymd_opt(naive_date.year() + 1, 1, 1)
    } else {
        NaiveDate::from_ymd_opt(naive_date.year(), naive_date.month() + 1, 1)
    }.unwrap();
    let end_date = Utc.from_utc_datetime(&(next_month - Duration::days(1)).and_hms_opt(23, 59, 59).unwrap());
    log::info!("Target range - Start: {}, End: {}", start_date, end_date);

    let client = Client::builder()
        .user_agent("rust-github-contributions")
        .build()?;
    let initial_url = format!(
        "https://api.github.com/users/{}/events?per_page=100",
        username
    );
    log::debug!("Initial API URL: {}", initial_url);

    let mut all_events: Vec<GitHubEvent> = Vec::new();
    let mut page_url = initial_url.clone();
    let mut has_next = true;

    while has_next {
        log::debug!("Fetching page: {}", page_url);
        let mut request = client
            .get(&page_url)
            .header("Accept", "application/vnd.github.v3+json");
        
        if !token.is_empty() {
            request = request.header("Authorization", format!("Bearer {}", token));
            log::debug!("Adding Authorization header with token");
        }

        let response = request.send()?;
        log::debug!("Response status: {}", response.status());

        if !response.status().is_success() {
            log::error!("API request failed with status: {}", response.status());
            let error_body = response.text()?;
            log::error!("Error response body: {:?}", error_body);
            return Ok(());
        }

        let link_header = response.headers().get("Link").map(|h| h.to_str().unwrap_or("").to_string());
        log::debug!("Link header: {:?}", link_header);

        let page_events: Vec<GitHubEvent> = response.json()?;
        log::info!("Events received this page: {}", page_events.len());
        all_events.extend(page_events);

        has_next = false;
        if let Some(link_str) = link_header {
            if link_str.contains("rel=\"next\"") {
                let next_url = link_str.split(',').find(|s| s.contains("rel=\"next\""))
                    .and_then(|s| s.split(';').next())
                    .and_then(|s| s.trim().strip_prefix('<').and_then(|s| s.strip_suffix('>')))
                    .map(String::from);
                if let Some(next) = next_url {
                    page_url = next;
                    has_next = true;
                }
            }
        }

        if !all_events.is_empty() {
            let oldest_time = DateTime::parse_from_rfc3339(&all_events.last().unwrap().created_at)?;
            if oldest_time < start_date {
                log::debug!("Oldest event ({}) is before start date, stopping fetch", oldest_time);
                break;
            }
        }
    }

    log::info!("Total events received: {}", all_events.len());
    log::trace!("Raw events: {:?}", all_events);

    // Filter events and fetch additional details
    let mut daily_summaries: HashMap<String, Vec<(GitHubEvent, Vec<CommitDetail>, Option<PullRequestDetail>)>> = HashMap::new();
    log::info!("Filtering events for range {} to {}", start_date, end_date);
    for event in &all_events {
        let event_time = DateTime::parse_from_rfc3339(&event.created_at)?;
        let in_range = event_time >= start_date && event_time <= end_date;
        let mut commits = Vec::new();
        let mut pr_detail = None;

        if event.event_type == "PushEvent" && in_range {
            if let Some(commits_array) = event.payload.get("commits").and_then(|v| v.as_array()) {
                for commit in commits_array {
                    if let Some(sha) = commit.get("sha").and_then(|v| v.as_str()) {
                        let commit_url = format!(
                            "https://api.github.com/repos/{}/commits/{}",
                            event.repo.name, sha
                        );
                        let mut commit_request = client.get(&commit_url)
                            .header("Accept", "application/vnd.github.v3+json");
                        if !token.is_empty() {
                            commit_request = commit_request.header("Authorization", format!("Bearer {}", token));
                        }
                        let commit_response = commit_request.send()?;
                        if commit_response.status().is_success() {
                            let commit_detail: CommitDetail = commit_response.json()?;
                            commits.push(commit_detail);
                        } else {
                            log::warn!("Failed to fetch commit {}: {}", sha, commit_response.status());
                        }
                    }
                }
            }
        } else if event.event_type == "PullRequestEvent" && in_range {
            if let Some(number) = event.payload.get("number").and_then(|v| v.as_i64()) {
                let pr_url = format!(
                    "https://api.github.com/repos/{}/pulls/{}",
                    event.repo.name, number
                );
                let mut pr_request = client.get(&pr_url)
                    .header("Accept", "application/vnd.github.v3+json");
                if !token.is_empty() {
                    pr_request = pr_request.header("Authorization", format!("Bearer {}", token));
                }
                let pr_response = pr_request.send()?;
                if pr_response.status().is_success() {
                    let pr: PullRequestDetail = pr_response.json()?;
                    pr_detail = Some(pr);
                } else {
                    log::warn!("Failed to fetch PR #{}: {}", number, pr_response.status());
                }
            }
        }

        log::debug!(
            "Event - Time: {}, Type: {}, Repo: {}, In range: {}",
            event.created_at, event.event_type, event.repo.name, in_range
        );
        
        if in_range {
            let day_key = event_time.format("%Y-%m-%d").to_string();
            log::debug!("Adding event to day: {}", day_key);
            daily_summaries
                .entry(day_key)
                .or_insert_with(Vec::new)
                .push((event.clone(), commits, pr_detail));
        }
    }

    log::debug!("Daily summaries after filtering: {:?}", daily_summaries);
    if daily_summaries.is_empty() {
        log::warn!("No events found for {} in the specified range.", username);
        return Ok(());
    }

    println!("----------------------------------------");
    let days_in_month = (end_date - start_date).num_days() as u32 + 1;
    log::debug!("Days in month: {}", days_in_month);
    for day in 1..=days_in_month {
        let current_date = NaiveDate::from_ymd_opt(start_date.year(), start_date.month(), day);
        if let Some(date) = current_date {
            let date_str = date.format("%Y-%m-%d").to_string();
            log::debug!("Checking day: {}", date_str);
            
            if let Some(events) = daily_summaries.get(&date_str) {
                let mut sorted_events: Vec<&(GitHubEvent, Vec<CommitDetail>, Option<PullRequestDetail>)> = events.iter().collect();
                sorted_events.sort_by(|a, b| a.0.created_at.cmp(&b.0.created_at));
                log::debug!("Events for {}: {:?}", date_str, sorted_events);

                let start_time = DateTime::parse_from_rfc3339(&sorted_events[0].0.created_at)?
                    .format("%H:%M:%S UTC");
                let end_time = DateTime::parse_from_rfc3339(&sorted_events.last().unwrap().0.created_at)?
                    .format("%H:%M:%S UTC");

                println!("\n{}:", date_str);
                println!("Start time: {}", start_time);
                println!("End time: {}", end_time);
                println!("Contributions ({}):", events.len());
                
                for (event, commits, pr_detail) in events {
                    println!("- {}: {}", event.event_type, event.repo.name);
                    if event.event_type == "PushEvent" && !commits.is_empty() {
                        for commit in commits {
                            println!("  Commit {}: {}", commit.sha, commit.commit.message);
                        }
                    } else if event.event_type == "PullRequestEvent" && pr_detail.is_some() {
                        let pr = pr_detail.as_ref().unwrap();
                        let action = event.payload.get("action").and_then(|v| v.as_str()).unwrap_or("unknown");
                        println!("  PR #{}: {} (Action: {}, State: {}, Merged: {})", 
                            pr.number, pr.title, action, pr.state, pr.merged);
                        if let Some(body) = &pr.body {
                            println!("    Description: {}", body);
                        }
                    }
                }
            } else {
                log::debug!("No events found for {}", date_str);
            }
        } else {
            log::error!("Invalid date generated for day: {}", day);
        }
    }

    Ok(())
}