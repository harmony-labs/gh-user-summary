use chrono::{DateTime, Utc, Datelike, Duration, NaiveDate, TimeZone};
use serde::Deserialize;
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
}

#[derive(Deserialize, Debug, Clone)]
struct Repository {
    name: String,
}

fn main() -> Result<(), Box<dyn Error>> {
    // Initialize logging from logging.rs
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

    // Check token
    let token = env::var("GITHUB_TOKEN").unwrap_or_default();
    if token.is_empty() {
        log::warn!("No GITHUB_TOKEN found in environment");
    } else {
        log::debug!("GITHUB_TOKEN found (length: {})", token.len());
    }

    // Parse date range
    log::debug!("Parsing date range for {}", month_input);
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

    // Build API client
    let client = Client::builder()
        .user_agent("rust-github-contributions")
        .build()?;
    let initial_url = format!(
        "https://api.github.com/users/{}/events?per_page=100",
        username
    );
    log::debug!("Initial API URL: {}", initial_url);

    // Fetch events with pagination
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
        } else {
            log::warn!("No token, proceeding with unauthenticated request");
        }

        let response = request.send()?;
        log::debug!("Response status: {}", response.status());

        if !response.status().is_success() {
            log::error!("API request failed with status: {}", response.status());
            let error_body = response.text()?;
            log::error!("Error response body: {:?}", error_body);
            return Ok(());
        }

        // Get headers before consuming response
        let link_header = response.headers().get("Link").map(|h| h.to_str().unwrap_or("").to_string());
        log::debug!("Link header: {:?}", link_header);

        let page_events: Vec<GitHubEvent> = response.json()?;
        log::info!("Events received this page: {}", page_events.len());
        all_events.extend(page_events);

        // Check for next page
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

        // Stop if we've gone far enough back
        if !all_events.is_empty() {
            let oldest_time = DateTime::parse_from_rfc3339(&all_events.last().unwrap().created_at)?;
            if oldest_time < start_date {
                log::debug!("Oldest event ({}) is before start date, stopping fetch", oldest_time);
                break;
            }
        }
    }

    log::info!("Total events received: {}", all_events.len());
    log::debug!("Raw events: {:?}", all_events);

    // Filter events
    let mut daily_summaries: HashMap<String, Vec<GitHubEvent>> = HashMap::new();
    log::info!("Filtering events for range {} to {}", start_date, end_date);
    for event in &all_events {
        let event_time = DateTime::parse_from_rfc3339(&event.created_at)?;
        let in_range = event_time >= start_date && event_time <= end_date;
        log::debug!(
            "Event - Time: {}, Type: {}, Repo: {}, In range: {}",
            event_time, event.event_type, event.repo.name, in_range
        );
        
        if in_range {
            let day_key = event_time.format("%Y-%m-%d").to_string();
            log::debug!("Adding event to day: {}", day_key);
            daily_summaries
                .entry(day_key)
                .or_insert_with(Vec::new)
                .push(event.clone());
        }
    }

    log::debug!("Daily summaries after filtering: {:?}", daily_summaries);
    if daily_summaries.is_empty() {
        log::warn!("No events found for {} in the specified range.", username);
        return Ok(());
    }

    // Generate summary
    log::info!("GitHub Contributions for {} in {}", username, month_input);
    println!("----------------------------------------");

    let days_in_month = (end_date - start_date).num_days() as u32 + 1;
    log::debug!("Days in month: {}", days_in_month);
    for day in 1..=days_in_month {
        let current_date = NaiveDate::from_ymd_opt(start_date.year(), start_date.month(), day);
        if let Some(date) = current_date {
            let date_str = date.format("%Y-%m-%d").to_string();
            log::debug!("Checking day: {}", date_str);
            
            if let Some(events) = daily_summaries.get(&date_str) {
                let mut sorted_events: Vec<&GitHubEvent> = events.iter().collect();
                sorted_events.sort_by(|a, b| a.created_at.cmp(&b.created_at));
                log::debug!("Events for {}: {:?}", date_str, sorted_events);

                let start_time = DateTime::parse_from_rfc3339(&sorted_events[0].created_at)?
                    .format("%H:%M:%S UTC");
                let end_time = DateTime::parse_from_rfc3339(&sorted_events.last().unwrap().created_at)?
                    .format("%H:%M:%S UTC");

                println!("\n{}:", date_str);
                println!("Start time: {}", start_time);
                println!("End time: {}", end_time);
                println!("Contributions ({}):", events.len());
                
                for event in events {
                    println!("- {}: {}", event.event_type, event.repo.name);
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