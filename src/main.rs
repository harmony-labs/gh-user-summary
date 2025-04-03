use chrono::{NaiveDate, Utc, Duration, TimeZone, Datelike};
use clap::Parser;
use std::error::Error;
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
}

fn main() -> Result<(), Box<dyn Error>> {
    logging::init_logging().expect("Failed to initialize logging");
    log::debug!("Starting gh-user-summary...");

    let args = Args::parse();
    log::debug!("Command line args: {:?}", args);

    let token = std::env::var("GITHUB_TOKEN").unwrap_or_default();
    if token.is_empty() {
        log::warn!("No GITHUB_TOKEN found in environment");
    } else {
        log::debug!("GITHUB_TOKEN found (length: {})", token.len());
    }

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

    log::info!("Target range - Start: {}, End: {}", start_date, end_date);

    let events = api::fetch_all_events(&client, &args.username, &token, start_date)?;
    let daily_summaries = events::process_events(&client, &token, events, start_date, end_date)?;
    summary::print_summaries(daily_summaries, start_date, end_date)?;

    Ok(())
}