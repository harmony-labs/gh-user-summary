use chrono::{NaiveDate, Utc, Duration, TimeZone, Datelike}; // Added Datelike
use std::env;
use std::error::Error;
mod api;
mod events;
mod logging;

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

    let client = api::create_client()?;
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

    let events = api::fetch_all_events(&client, username, &token, start_date)?;
    let daily_summaries = events::process_events(&client, &token, events, start_date, end_date)?;
    events::print_summaries(daily_summaries, start_date, end_date)?;

    Ok(())
}