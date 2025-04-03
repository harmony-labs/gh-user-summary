use chrono::{DateTime, Utc};
use reqwest::blocking::Client;
use serde::Deserialize;
use serde_json::{Value, to_vec, from_slice};
use std::error::Error;
use cacache;

#[derive(Deserialize, Debug, Clone)]
pub struct GitHubEvent {
    pub created_at: String,
    #[serde(rename = "type")]
    pub event_type: String,
    pub repo: Repository,
    pub payload: Value,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Repository {
    pub name: String,
}

#[derive(Deserialize, Debug)]
pub struct CommitDetail {
    pub sha: String,
    pub commit: CommitInfo,
}

#[derive(Deserialize, Debug)]
pub struct CommitInfo {
    pub message: String,
}

#[derive(Deserialize, Debug)]
pub struct PullRequestDetail {
    pub number: i32,
    pub title: String,
    pub body: Option<String>,
    pub state: String,
    pub merged: bool,
    pub html_url: String,
}

pub fn create_client() -> Result<Client, Box<dyn Error>> {
    let client = reqwest::blocking::Client::builder()
        .user_agent("rust-github-contributions")
        .build()?;
    log::debug!("Initialized client with disk caching at ./cache");
    Ok(client)
}

fn fetch_and_cache<T: serde::de::DeserializeOwned>(client: &Client, url: &str, token: &str, cache_key: &str) -> Result<T, Box<dyn Error>> {
    let cache_dir = "./.cache";

    // Check cache first
    if let Ok(cached_data) = cacache::read_sync(cache_dir, cache_key) {
        let result: T = from_slice(&cached_data)?;
        log::debug!("Cache hit for {}", url);
        return Ok(result);
    }

    // Fetch from API
    let mut request = client.get(url)
        .header("Accept", "application/vnd.github.v3+json");
    if !token.is_empty() {
        request = request.header("Authorization", format!("Bearer {}", token));
    }
    let response = request.send()?;
    if !response.status().is_success() {
        log::error!("API request failed for {}: {}", url, response.status());
        let error_body = response.text()?;
        log::error!("Error response body: {:?}", error_body);
        return Err("API request failed".into());
    }

    // Cache the response
    let bytes = response.bytes()?;
    let result: T = from_slice(&bytes)?;
    cacache::write_sync(cache_dir, cache_key, &bytes)?;
    log::debug!("Fetched and cached {}", url);
    Ok(result)
}

pub fn fetch_all_events(client: &Client, username: &str, token: &str, start_date: DateTime<Utc>) -> Result<Vec<GitHubEvent>, Box<dyn Error>> {
    let mut all_events: Vec<GitHubEvent> = Vec::new();
    let initial_url = format!("https://api.github.com/users/{}/events?per_page=100", username);
    let mut page_url = initial_url.clone();
    let mut has_next = true;
    let mut page_count = 0;

    while has_next {
        page_count += 1;
        log::debug!("Fetching page {}: {}", page_count, page_url);
        let cache_key = format!("events:{}", page_url);
        let page_events: Vec<GitHubEvent> = fetch_and_cache(client, &page_url, token, &cache_key)?;
        log::debug!("Events received this page: {}", page_events.len());
        all_events.extend(page_events);

        let response = client.get(&page_url)
            .header("Accept", "application/vnd.github.v3+json")
            .header("Authorization", format!("Bearer {}", token))
            .send()?;
        let link_header = response.headers().get("Link").map(|h| h.to_str().unwrap_or("").to_string());
        log::debug!("Link header for page {}: {:?}", page_count, link_header);

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
            // Check if we’ve reached the last page
            if !link_str.contains("rel=\"last\"") && all_events.len() >= 300 && !has_next {
                log::warn!(
                    "Fetched 300 events across {} pages, but no 'last' link found. Data might be truncated.",
                    page_count
                );
            }
        }

        if !all_events.is_empty() {
            let oldest_time = DateTime::parse_from_rfc3339(&all_events.last().unwrap().created_at)?;
            if oldest_time < start_date && has_next {
                log::debug!(
                    "Oldest event ({}) is before start date ({}), but more pages exist. Continuing fetch.",
                    oldest_time, start_date
                );
            } else if oldest_time < start_date {
                log::debug!(
                    "Oldest event ({}) is before start date ({}), stopping fetch.",
                    oldest_time, start_date
                );
                break;
            }
        }
    }

    log::info!("Total pages fetched: {}", page_count);
    log::debug!("Total events received: {}", all_events.len());
    log::trace!("Raw events: {:?}", all_events);
    Ok(all_events)
}

pub fn fetch_commit_detail(client: &Client, token: &str, repo: &str, sha: &str) -> Result<CommitDetail, Box<dyn Error>> {
    let commit_url = format!("https://api.github.com/repos/{}/commits/{}", repo, sha);
    let cache_key = format!("commit:{}:{}", repo, sha);
    fetch_and_cache(client, &commit_url, token, &cache_key)
}

pub fn fetch_pr_detail(client: &Client, token: &str, repo: &str, number: i64) -> Result<PullRequestDetail, Box<dyn Error>> {
    let pr_url = format!("https://api.github.com/repos/{}/pulls/{}", repo, number);
    let cache_key = format!("pr:{}:{}", repo, number);
    fetch_and_cache(client, &pr_url, token, &cache_key)
}