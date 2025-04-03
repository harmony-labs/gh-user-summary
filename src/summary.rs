use chrono::{DateTime, Utc, NaiveDate, Datelike};
use std::collections::HashMap;
use std::error::Error;
use crate::api::{GitHubEvent, CommitDetail, PullRequestDetail};

pub fn print_summaries(
    daily_summaries: HashMap<String, Vec<(GitHubEvent, Vec<CommitDetail>, Option<PullRequestDetail>)>>,
    start_date: DateTime<Utc>,
    end_date: DateTime<Utc>,
) -> Result<(), Box<dyn Error>> {
    println!("# GitHub Contributions Summary\n");
    println!("*Date Range: {} to {}*\n", start_date.format("%Y-%m-%d"), end_date.format("%Y-%m-%d"));

    // Calculate overall summary stats
    let total_events: usize = daily_summaries.values().map(|events| events.len()).sum();
    let active_days = daily_summaries.len();
    let mut event_types: HashMap<String, usize> = HashMap::new();
    for events in daily_summaries.values() {
        for (event, _, _) in events {
            *event_types.entry(event.event_type.clone()).or_insert(0) += 1;
        }
    }

    // Print overall summary
    println!("## Summary");
    println!("- **Total Events**: {}", total_events);
    println!("- **Active Days**: {}", active_days);
    let event_type_summary = event_types.iter()
        .map(|(type_name, count)| format!("{} {}", count, type_name))
        .collect::<Vec<String>>()
        .join(", ");
    println!("- **Event Types**: {}", if event_type_summary.is_empty() { "None".to_string() } else { event_type_summary });
    println!();

    let days_in_month = (end_date - start_date).num_days() as u32 + 1;
    log::debug!("Days in range: {}", days_in_month);

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

                // Calculate daily event type counts
                let mut daily_event_types: HashMap<String, usize> = HashMap::new();
                for (event, _, _) in events {
                    *daily_event_types.entry(event.event_type.clone()).or_insert(0) += 1;
                }
                let daily_event_summary = daily_event_types.iter()
                    .map(|(type_name, count)| format!("{} {}", count, type_name))
                    .collect::<Vec<String>>()
                    .join(", ");

                println!("## {}\n", date_str);
                println!("- **Start Time**: {}", start_time);
                println!("- **End Time**: {}", end_time);
                println!("- **Contributions**: {} event(s)", events.len());
                println!("- **Event Types**: {}", daily_event_summary);
                println!();

                for (event, commits, pr_detail) in events {
                    println!("- **{}** - `{}`", event.event_type, event.repo.name);
                    if event.event_type == "PushEvent" && !commits.is_empty() {
                        for commit in commits {
                            let message_lines = commit.commit.message.split('\n');
                            let mut first_line = true;
                            for line in message_lines {
                                if first_line {
                                    println!("  - Commit `{}`: {}", commit.sha, line);
                                    first_line = false;
                                } else {
                                    println!("    {}", line);
                                }
                            }
                        }
                    } else if event.event_type == "PullRequestEvent" && pr_detail.is_some() {
                        let pr = pr_detail.as_ref().unwrap();
                        let action = event.payload.get("action").and_then(|v| v.as_str()).unwrap_or("unknown");
                        println!("  - PR [#{}]({}): {} (Action: {}, State: {}, Merged: {})",
                            pr.number, pr.html_url, pr.title, action, pr.state, pr.merged);
                    } else if event.event_type == "CreateEvent" {
                        let ref_name = event.payload.get("ref").and_then(|v| v.as_str()).unwrap_or("none");
                        let ref_type = event.payload.get("ref_type").and_then(|v| v.as_str()).unwrap_or("unknown");
                        println!("  - Created {}: `{}`", ref_type, ref_name);
                    } else if event.event_type == "DeleteEvent" {
                        let ref_name = event.payload.get("ref").and_then(|v| v.as_str()).unwrap_or("none");
                        let ref_type = event.payload.get("ref_type").and_then(|v| v.as_str()).unwrap_or("unknown");
                        println!("  - Deleted {}: `{}`", ref_type, ref_name);
                    }
                    println!(); // Empty line for spacing between events
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