use chrono::{DateTime, Utc};
use reqwest::blocking::{Client, ClientBuilder};
use serde::Deserialize;
use serde_json::Value;
use std::error::Error;

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
}

pub fn create_client() -> Result<Client, Box<dyn Error>> {
    let client = ClientBuilder::new()
        .user_agent("rust-github-contributions")
        .build()?;
    Ok(client)
}

pub fn fetch_all_events(client: &Client, username: &str, token: &str, start_date: DateTime<Utc>) -> Result<Vec<GitHubEvent>, Box<dyn Error>> {
    let mut all_events: Vec<GitHubEvent> = Vec::new();
    let initial_url = format!("https://api.github.com/users/{}/events?per_page=100", username);
    let mut page_url = initial_url.clone();
    let mut has_next = true;

    while has_next {
        log::debug!("Fetching page: {}", page_url);
        let mut request = client.get(&page_url)
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
            return Err("API request failed".into());
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
    Ok(all_events)
}

pub fn fetch_commit_detail(client: &Client, token: &str, repo: &str, sha: &str) -> Result<CommitDetail, Box<dyn Error>> {
    let commit_url = format!("https://api.github.com/repos/{}/commits/{}", repo, sha);
    let mut request = client.get(&commit_url)
        .header("Accept", "application/vnd.github.v3+json");
    if !token.is_empty() {
        request = request.header("Authorization", format!("Bearer {}", token));
    }
    let response = request.send()?;
    if response.status().is_success() {
        let commit_detail: CommitDetail = response.json()?;
        Ok(commit_detail)
    } else {
        log::warn!("Failed to fetch commit {}: {}", sha, response.status());
        Err(format!("Failed to fetch commit: {}", response.status()).into())
    }
}

pub fn fetch_pr_detail(client: &Client, token: &str, repo: &str, number: i64) -> Result<PullRequestDetail, Box<dyn Error>> {
    let pr_url = format!("https://api.github.com/repos/{}/pulls/{}", repo, number);
    let mut request = client.get(&pr_url)
        .header("Accept", "application/vnd.github.v3+json");
    if !token.is_empty() {
        request = request.header("Authorization", format!("Bearer {}", token));
    }
    let response = request.send()?;
    if response.status().is_success() {
        let pr_detail: PullRequestDetail = response.json()?;
        Ok(pr_detail)
    } else {
        log::warn!("Failed to fetch PR #{}: {}", number, response.status());
        Err(format!("Failed to fetch PR: {}", response.status()).into())
    }
}