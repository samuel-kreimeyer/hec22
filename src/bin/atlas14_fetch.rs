//! NOAA ATLAS14 Rainfall Data Fetcher
//!
//! Fetches precipitation frequency data from NOAA ATLAS14 database
//! and outputs IDF curves in CSV format compatible with HEC-22 analysis.
//!
//! ## Usage
//!
//! Using coordinates:
//! ```bash
//! atlas14_fetch --lat 40.7128 --lon -74.0060 --output nyc_idf.csv
//! ```
//!
//! The output CSV will have columns: return_period, duration, intensity

use clap::Parser;
use reqwest::blocking::Client;
use std::error::Error;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "atlas14_fetch")]
#[command(version = "0.1.0")]
#[command(about = "Fetch NOAA ATLAS14 rainfall data and generate IDF curves", long_about = None)]
struct Cli {
    /// Latitude (decimal degrees, e.g., 40.7128)
    #[arg(short = 'y', long)]
    lat: f64,

    /// Longitude (decimal degrees, e.g., -74.0060)
    #[arg(short = 'x', long)]
    lon: f64,

    /// Output CSV file path
    #[arg(short, long)]
    output: PathBuf,

    /// Unit system (english or metric)
    #[arg(short, long, default_value = "english")]
    units: String,

    /// Return periods to fetch (comma-separated, e.g., "2,5,10,25,50,100")
    #[arg(short, long, default_value = "2,5,10,25,50,100")]
    return_periods: String,

    /// Durations in minutes (comma-separated, e.g., "5,10,15,30,60,120")
    #[arg(short, long, default_value = "5,10,15,30,60,120")]
    durations: String,
}

/// IDF curve entry
#[derive(Debug, Clone)]
struct IdfEntry {
    return_period: f64,
    duration: f64,
    intensity: f64,
}

fn main() -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();

    // Validate coordinates
    if cli.lat < -90.0 || cli.lat > 90.0 {
        return Err("Latitude must be between -90 and 90 degrees".into());
    }
    if cli.lon < -180.0 || cli.lon > 180.0 {
        return Err("Longitude must be between -180 and 180 degrees".into());
    }

    println!("Fetching NOAA ATLAS14 data for coordinates: {}, {}", cli.lat, cli.lon);
    println!("This may take a moment...\n");

    // Parse return periods and durations
    let return_periods: Vec<f64> = cli
        .return_periods
        .split(',')
        .filter_map(|s| s.trim().parse().ok())
        .collect();

    let durations: Vec<f64> = cli
        .durations
        .split(',')
        .filter_map(|s| s.trim().parse().ok())
        .collect();

    // Fetch data from NOAA ATLAS14
    let idf_data = fetch_atlas14_data(&cli.lat, &cli.lon, &cli.units, &return_periods, &durations)?;

    // Write to CSV
    write_idf_csv(&cli.output, &idf_data)?;

    println!("✓ IDF curve data written to: {}", cli.output.display());
    println!("  {} return periods × {} durations = {} data points",
             return_periods.len(),
             durations.len(),
             idf_data.len());

    Ok(())
}

/// Fetch ATLAS14 data from NOAA web service
fn fetch_atlas14_data(
    lat: &f64,
    lon: &f64,
    units: &str,
    return_periods: &[f64],
    durations: &[f64],
) -> Result<Vec<IdfEntry>, Box<dyn Error>> {
    let client = Client::builder()
        .timeout(std::time::Duration::from_secs(60))
        .build()?;

    // NOAA HDSC Precipitation Frequency Data Server API
    // This endpoint returns CSV data with precipitation frequency estimates
    let url = format!(
        "https://hdsc.nws.noaa.gov/cgi-bin/new/fe_text_lwr.csv?lat={:.4}&lon={:.4}&data=intensity&units={}&series=pds",
        lat, lon, units
    );

    println!("Querying NOAA ATLAS14 API...");
    println!("URL: {}\n", url);

    let response = client.get(&url).send()?;

    if !response.status().is_success() {
        return Err(format!("API request failed with status: {}", response.status()).into());
    }

    let csv_text = response.text()?;

    println!("Successfully retrieved NOAA ATLAS14 data");
    println!("Parsing precipitation frequency estimates...\n");

    // Parse the CSV response from NOAA
    parse_noaa_csv(&csv_text, return_periods, durations)
}

/// Parse NOAA ATLAS14 CSV response
///
/// NOAA returns CSV with headers and data in a specific format.
/// The CSV contains precipitation intensity values for various durations (columns)
/// and return periods (rows).
fn parse_noaa_csv(
    csv_text: &str,
    requested_return_periods: &[f64],
    requested_durations: &[f64],
) -> Result<Vec<IdfEntry>, Box<dyn Error>> {
    let mut idf_entries = Vec::new();
    let mut csv_reader = csv::ReaderBuilder::new()
        .has_headers(true)
        .flexible(true)
        .from_reader(csv_text.as_bytes());

    // Read headers to get duration values
    let headers = csv_reader.headers()?;

    // First column is typically the return period label
    // Subsequent columns are durations (e.g., "5-min", "10-min", "15-min", etc.)
    let mut duration_map: Vec<(usize, f64)> = Vec::new();

    for (idx, header) in headers.iter().enumerate().skip(1) {
        // Parse duration from header (e.g., "5-min" -> 5.0)
        if let Some(duration) = parse_duration_from_header(header) {
            duration_map.push((idx, duration));
        }
    }

    // Parse each row (each row represents a return period)
    for result in csv_reader.records() {
        let record = result?;

        // First column should be the return period
        let return_period_str = record.get(0).unwrap_or("");
        let return_period = parse_return_period(return_period_str)?;

        // Only process requested return periods
        if !requested_return_periods.contains(&return_period) {
            continue;
        }

        // Extract intensities for each duration
        for (col_idx, duration) in &duration_map {
            // Only process requested durations
            if !requested_durations.contains(duration) {
                continue;
            }

            if let Some(value_str) = record.get(*col_idx) {
                if let Ok(intensity) = value_str.trim().parse::<f64>() {
                    idf_entries.push(IdfEntry {
                        return_period,
                        duration: *duration,
                        intensity,
                    });
                }
            }
        }
    }

    if idf_entries.is_empty() {
        return Err("No data extracted from NOAA response. Check lat/lon coordinates and parameters.".into());
    }

    Ok(idf_entries)
}

/// Parse duration from CSV header (e.g., "5-min" -> 5.0, "1-hr" -> 60.0)
fn parse_duration_from_header(header: &str) -> Option<f64> {
    let header_lower = header.to_lowercase();

    // Handle various formats: "5-min", "5 min", "5min", "1-hr", "1 hr"
    if header_lower.contains("min") {
        // Extract number before "min"
        let num_str: String = header_lower
            .chars()
            .take_while(|c| c.is_numeric() || *c == '.')
            .collect();
        num_str.parse::<f64>().ok()
    } else if header_lower.contains("hr") || header_lower.contains("hour") {
        // Extract number and convert hours to minutes
        let num_str: String = header_lower
            .chars()
            .take_while(|c| c.is_numeric() || *c == '.')
            .collect();
        num_str.parse::<f64>().ok().map(|h| h * 60.0)
    } else if header_lower.contains("day") {
        // Extract number and convert days to minutes
        let num_str: String = header_lower
            .chars()
            .take_while(|c| c.is_numeric() || *c == '.')
            .collect();
        num_str.parse::<f64>().ok().map(|d| d * 1440.0)
    } else {
        None
    }
}

/// Parse return period from row header (e.g., "10-yr" -> 10.0, "100" -> 100.0)
fn parse_return_period(s: &str) -> Result<f64, Box<dyn Error>> {
    let s_lower = s.to_lowercase();

    // Extract numeric part
    let num_str: String = s_lower
        .chars()
        .filter(|c| c.is_numeric() || *c == '.')
        .collect();

    num_str.parse::<f64>()
        .map_err(|e| format!("Failed to parse return period from '{}': {}", s, e).into())
}

/// Write IDF data to CSV file in HEC-22 format
fn write_idf_csv(path: &PathBuf, data: &[IdfEntry]) -> Result<(), Box<dyn Error>> {
    let mut file = File::create(path)?;

    // Write header
    writeln!(file, "return_period,duration,intensity")?;

    // Write data rows, sorted by return period then duration
    let mut sorted_data = data.to_vec();
    sorted_data.sort_by(|a, b| {
        a.return_period
            .partial_cmp(&b.return_period)
            .unwrap()
            .then(a.duration.partial_cmp(&b.duration).unwrap())
    });

    for entry in sorted_data {
        writeln!(
            file,
            "{},{},{:.2}",
            entry.return_period as i32,
            entry.duration as i32,
            entry.intensity
        )?;
    }

    Ok(())
}
