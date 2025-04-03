// src/main.rs
use chrono::{NaiveDate, Utc, Duration, TimeZone, Datelike};
use clap::Parser;
use std::error::Error;
mod ai;
mod api;
mod events;
mod logging;
mod summary;

#[derive(Parser, Debug)]
#[command(version, about = "Summarize GitHub contributions", long_about = None)]
struct Args {
    /// GitHub username (required)
    #[arg(short = 'u', long, required = true)]
    username: String,

    /// Start date (YYYY-MM-DD), mutually exclusive with -m and -d
    #[arg(short = 's', long, conflicts_with_all = &["month", "day"])]
    start_date: Option<String>,

    /// End date (YYYY-MM-DD), mutually exclusive with -m and -d
    #[arg(short = 'e', long, conflicts_with_all = &["month", "day"])]
    end_date: Option<String>,

    /// Month (YYYY-MM), sets start and end dates for the month
    #[arg(short = 'm', long, conflicts_with_all = &["start_date", "end_date", "day"])]
    month: Option<String>,

    /// Day (YYYY-MM-DD), sets start and end dates to that day
    #[arg(short = 'd', long, conflicts_with_all = &["start_date", "end_date", "month"])]
    day: Option<String>,

    /// AI provider to use (deepseek, openai, anthropic)
    #[arg(long)]
    ai_provider: Option<String>,
}

fn main() -> Result<(), Box<dyn Error>> {
    logging::init_logging().expect("Failed to initialize logging");
    log::debug!("Starting gh-user-summary...");

    let args = Args::parse();
    log::debug!("Command line args: {:?}", args);

    let github_token = std::env::var("GITHUB_TOKEN").unwrap_or_default();
    if github_token.is_empty() {
        log::warn!("No GITHUB_TOKEN found in environment");
    } else {
        log::debug!("GITHUB_TOKEN found (length: {})", github_token.len());
    }

    // Fetch API tokens for all providers
    let deepseek_token = std::env::var("DEEPSEEK_API_TOKEN").unwrap_or_default();
    let openai_token = std::env::var("OPENAI_API_KEY").unwrap_or_default();
    let anthropic_token = std::env::var("ANTHROPIC_API_KEY").unwrap_or_default();

    // Count available tokens
    let token_count = [!deepseek_token.is_empty(), !openai_token.is_empty(), !anthropic_token.is_empty()]
        .iter()
        .filter(|&&x| x)
        .count();

    // Determine the AI provider and token
    let (provider, ai_token) = match args.ai_provider.as_ref().map(|s| ai::AiProvider::from_str(s)) {
        Some(Some(p)) => {
            // User specified a provider explicitly
            let token = match p {
                ai::AiProvider::DeepSeek => deepseek_token,
                ai::AiProvider::OpenAI => openai_token,
                ai::AiProvider::Anthropic => anthropic_token,
            };
            if token.is_empty() {
                log::warn!(
                    "No {} found for specified provider '{:?}'; AI response generation will be skipped",
                    match p {
                        ai::AiProvider::DeepSeek => "DEEPSEEK_API_TOKEN",
                        ai::AiProvider::OpenAI => "OPENAI_API_KEY",
                        ai::AiProvider::Anthropic => "ANTHROPIC_API_KEY",
                    },
                    p
                );
            } else {
                log::debug!("Using specified provider {:?} with token (length: {})", p, token.len());
            }
            (p, token)
        }
        _ => {
            // No explicit provider or invalid provider specified; check tokens
            match token_count {
                1 => {
                    // Exactly one token is set; default to that provider
                    if !deepseek_token.is_empty() {
                        log::debug!("Defaulting to DeepSeek (only token found, length: {})", deepseek_token.len());
                        (ai::AiProvider::DeepSeek, deepseek_token)
                    } else if !openai_token.is_empty() {
                        log::debug!("Defaulting to OpenAI (only token found, length: {})", openai_token.len());
                        (ai::AiProvider::OpenAI, openai_token)
                    } else {
                        log::debug!("Defaulting to Anthropic (only token found, length: {})", anthropic_token.len());
                        (ai::AiProvider::Anthropic, anthropic_token)
                    }
                }
                0 => {
                    log::warn!("No API tokens found (DEEPSEEK_API_TOKEN, OPENAI_API_KEY, ANTHROPIC_API_KEY); AI response generation will be skipped");
                    (ai::AiProvider::DeepSeek, String::new()) // Placeholder, won’t be used
                }
                _ => {
                    log::warn!(
                        "Multiple API tokens found ({} set); please specify --ai-provider (deepseek, openai, anthropic); AI response generation will be skipped",
                        token_count
                    );
                    (ai::AiProvider::DeepSeek, String::new()) // Placeholder, won’t be used
                }
            }
        }
    };

    let client = api::create_client()?;

    // Determine start_date and end_date based on args
    let (start_date, end_date) = match (&args.start_date, &args.end_date, &args.month, &args.day) {
        (Some(start), Some(end), None, None) => {
            let start_date = NaiveDate::parse_from_str(start, "%Y-%m-%d")?;
            let end_date = NaiveDate::parse_from_str(end, "%Y-%m-%d")?;
            (
                Utc.from_utc_datetime(&start_date.and_hms_opt(0, 0, 0).unwrap()),
                Utc.from_utc_datetime(&end_date.and_hms_opt(23, 59, 59).unwrap())
            )
        }
        (None, None, Some(month), None) => {
            let naive_date = NaiveDate::parse_from_str(&format!("{}-01", month), "%Y-%m-%d")?;
            let next_month = if naive_date.month() == 12 {
                NaiveDate::from_ymd_opt(naive_date.year() + 1, 1, 1)
            } else {
                NaiveDate::from_ymd_opt(naive_date.year(), naive_date.month() + 1, 1)
            }.unwrap();
            (
                Utc.from_utc_datetime(&naive_date.and_hms_opt(0, 0, 0).unwrap()),
                Utc.from_utc_datetime(&(next_month - Duration::days(1)).and_hms_opt(23, 59, 59).unwrap())
            )
        }
        (None, None, None, Some(day)) => {
            let naive_date = NaiveDate::parse_from_str(day, "%Y-%m-%d")?;
            (
                Utc.from_utc_datetime(&naive_date.and_hms_opt(0, 0, 0).unwrap()),
                Utc.from_utc_datetime(&naive_date.and_hms_opt(23, 59, 59).unwrap())
            )
        }
        (None, None, None, None) => {
            log::error!("Must provide --start-date and --end-date, --month, or --day");
            return Err("Missing date arguments".into());
        }
        _ => {
            log::error!("Invalid combination of date arguments");
            return Err("Invalid date arguments".into());
        }
    };

    log::debug!("Target range - Start: {}, End: {}", start_date, end_date);

    let events = api::fetch_all_events(&client, &args.username, &github_token, start_date)?;
    let daily_summaries = events::process_events(&client, &github_token, events, start_date, end_date)?;
    let summary_text = summary::generate_summary(daily_summaries, start_date, end_date)?;

    // Always print the original summary
    println!("{}", summary_text);

    // Attempt AI response generation
    let ai_result = ai::generate_ai_response(&client, &summary_text, provider, &ai_token);
    match ai_result {
        Ok(ai_response) => {
            println!("\n# AI-Generated Response\n");
            println!("{}", ai_response);
        }
        Err(e) => {
            log::warn!("Skipping AI response due to: {}", e);
            // Summary is already printed, so no further action needed
        }
    }

    Ok(())
}