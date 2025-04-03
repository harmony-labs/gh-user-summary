// src/ai.rs
use reqwest::blocking::Client;
use serde_json::{json, Value};
use std::error::Error;

#[derive(Debug, Clone, PartialEq)]
pub enum AiProvider {
    DeepSeek,
    OpenAI,
    Anthropic,
}

impl AiProvider {
    fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "deepseek" => Some(AiProvider::DeepSeek),
            "openai" => Some(AiProvider::OpenAI),
            "anthropic" => Some(AiProvider::Anthropic),
            _ => None,
        }
    }

    fn endpoint(&self) -> &str {
        match self {
            AiProvider::DeepSeek => "https://api.deepseek.com/v1/chat/completions",
            AiProvider::OpenAI => "https://api.openai.com/v1/chat/completions",
            AiProvider::Anthropic => "https://api.anthropic.com/v1/messages",
        }
    }

    fn default_model(&self) -> &str {
        match self {
            AiProvider::DeepSeek => "deepseek-chat",
            AiProvider::OpenAI => "gpt-4o-mini", // Cost-effective default
            AiProvider::Anthropic => "claude-3-5-sonnet-20241022",
        }
    }
}

pub fn generate_ai_response(client: &Client, summary: &str, provider: AiProvider, token: &str) -> Result<String, Box<dyn Error>> {
    if token.is_empty() {
        log::warn!("No API token found for {:?}; skipping AI response generation", provider);
        return Ok(format!("*(AI response skipped: No {} API token provided)*", match provider {
            AiProvider::DeepSeek => "DEEPSEEK_API_TOKEN",
            AiProvider::OpenAI => "OPENAI_API_KEY",
            AiProvider::Anthropic => "ANTHROPIC_API_KEY",
        }));
    }

    log::debug!("Sending summary to {:?} API (length: {} chars)", provider, summary.len());

    let url = provider.endpoint();
    let prompt = format!(
        "You are tasked with summarizing a GitHub contributions summary for technical managers and teammates. \
        Read the following markdown report, then write a concise, human-readable markdown response. \
        Provide a full daily report with context on what was worked on, extracted from all event data (e.g., commits, PRs, repo changes). \
        For each day, estimate the push activity window (start and end times) based on the earliest and latest event timestamps, \
        and calculate the approximate duration in hours. Note that these timestamps reflect when commits were pushed, not when work was done, \
        so this represents push activity, not total work time. Include key activities, productivity trends, and collaboration impact \
        in a professional yet approachable tone. Format it as markdown with daily sections (e.g., '### YYYY-MM-DD'):\n\n{}",
        summary
    );

    let mut request = client.post(url)
        .header("Authorization", format!("Bearer {}", token))
        .header("Content-Type", "application/json");

    let body = match provider {
        AiProvider::DeepSeek | AiProvider::OpenAI => json!({
            "model": provider.default_model(),
            "messages": [
                {"role": "system", "content": "You are a helpful AI that generates detailed, professional markdown summaries for technical audiences, including push activity estimates."},
                {"role": "user", "content": prompt}
            ],
            "max_tokens": 1000,
            "temperature": 0.7
        }),
        AiProvider::Anthropic => json!({
            "model": provider.default_model(),
            "messages": [
                {"role": "user", "content": prompt}
            ],
            "system": "You are a helpful AI that generates detailed, professional markdown summaries for technical audiences, including push activity estimates.",
            "max_tokens": 1000,
            "temperature": 0.7
        }),
    };

    // Anthropic requires an additional header
    if provider == AiProvider::Anthropic {
        request = request.header("x-api-key", token); // Anthropic uses x-api-key alongside Bearer
    }

    let response = request.json(&body).send()?;

    if !response.status().is_success() {
        let status = response.status();
        let error_text = response.text()?;
        log::error!("{:?} API request failed: {} - {}", provider, status, error_text);
        return Err(format!("{:?} API request failed: {}", provider, status).into());
    }

    let json: Value = response.json()?;
    let ai_response = match provider {
        AiProvider::DeepSeek | AiProvider::OpenAI => json
            .get("choices")
            .and_then(|choices| choices.get(0))
            .and_then(|choice| choice.get("message"))
            .and_then(|message| message.get("content")),
        AiProvider::Anthropic => json.get("content").and_then(|content| content.get(0)).and_then(|c| c.get("text")),
    }.and_then(|content| content.as_str())
        .ok_or(format!("Failed to parse {:?} API response", provider))?;

    log::debug!("Received AI response from {:?} (length: {} chars)", provider, ai_response.len());
    Ok(ai_response.trim().to_string())
}