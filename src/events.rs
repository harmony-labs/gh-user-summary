use chrono::{DateTime, Utc};
use reqwest::blocking::Client;
use std::collections::HashMap;
use std::error::Error;
use rayon::prelude::*;
use crate::api::{GitHubEvent, CommitDetail, PullRequestDetail, fetch_commit_detail, fetch_pr_detail};

pub fn process_events(
    client: &Client,
    token: &str,
    events: Vec<GitHubEvent>,
    start_date: DateTime<Utc>,
    end_date: DateTime<Utc>,
) -> Result<HashMap<String, Vec<(GitHubEvent, Vec<CommitDetail>, Option<PullRequestDetail>)>>, Box<dyn Error>> {
    log::info!("Filtering events for range {} to {}", start_date, end_date);

    let processed: Vec<(String, (GitHubEvent, Vec<CommitDetail>, Option<PullRequestDetail>))> = events
        .par_iter()
        .filter_map(|event| {
            let event_time = match DateTime::parse_from_rfc3339(&event.created_at) {
                Ok(time) => time,
                Err(e) => {
                    log::warn!("Failed to parse event time {}: {}", event.created_at, e);
                    return None;
                }
            };
            let in_range = event_time >= start_date && event_time <= end_date;

            log::debug!(
                "Event - Time: {}, Type: {}, Repo: {}, In range: {}",
                event.created_at, event.event_type, event.repo.name, in_range
            );

            if !in_range {
                return None;
            }

            let mut commits = Vec::new();
            let mut pr_detail = None;

            if event.event_type == "PushEvent" {
                if let Some(commits_array) = event.payload.get("commits").and_then(|v| v.as_array()) {
                    commits = commits_array
                        .par_iter()
                        .filter_map(|commit| {
                            commit.get("sha").and_then(|v| v.as_str()).map(|sha| {
                                match fetch_commit_detail(client, token, &event.repo.name, sha) {
                                    Ok(commit_detail) => Some(commit_detail),
                                    Err(e) => {
                                        log::warn!("Skipping commit fetch: {}", e);
                                        None
                                    }
                                }
                            }).flatten()
                        })
                        .collect();
                }
            } else if event.event_type == "PullRequestEvent" {
                if let Some(number) = event.payload.get("number").and_then(|v| v.as_i64()) {
                    pr_detail = match fetch_pr_detail(client, token, &event.repo.name, number) {
                        Ok(pr) => Some(pr),
                        Err(e) => {
                            log::warn!("Skipping PR fetch: {}", e);
                            None
                        }
                    };
                }
            }

            let day_key = event_time.format("%Y-%m-%d").to_string();
            log::debug!("Adding event to day: {}", day_key);
            Some((day_key, (event.clone(), commits, pr_detail)))
        })
        .collect();

    let mut daily_summaries: HashMap<String, Vec<(GitHubEvent, Vec<CommitDetail>, Option<PullRequestDetail>)>> = HashMap::new();
    for (day_key, event_data) in processed {
        daily_summaries
            .entry(day_key)
            .or_insert_with(Vec::new)
            .push(event_data);
    }

    log::debug!("Daily summaries after filtering: {:?}", daily_summaries);
    if daily_summaries.is_empty() {
        log::warn!("No events found in the specified range.");
    }
    Ok(daily_summaries)
}