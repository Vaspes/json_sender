use std::fs::File;
use std::fs::read_to_string;
use std::io::{BufRead, BufReader};
use std::time::Duration;
use std::thread::sleep;

use reqwest::blocking::Client;
use serde::Deserialize;
use clap::Parser;

#[derive(Deserialize)]
struct Config {
    api_url: String,
    auth_header: String,
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Path to the JSON file
    #[arg(short, long)]
    data_file: String,

    /// Path to the TOML configuration file
    #[arg(short, long)]
    config_file: String,

    /// Delay between requests in seconds
    #[arg(long, default_value_t = 1)]
    delay: u64,
}

fn main() {
    let args = Args::parse();

    // Load configuration
    let config_contents = read_to_string(args.config_file).expect("Error reading config file");
    let config: Config = toml::from_str(&config_contents).expect("Error parsing config file");

    // Read JSON data
    let file = File::open(args.data_file).expect("Error opening data file");
    let reader = BufReader::new(file);

    // HTTP client setup
    let client = Client::new();
    let content_type = "application/infracht.load.v1+json;charset=UTF-8";

    // Send requests with delay
    for line in reader.lines() {
        let json_payload = line.expect("Error reading line");

        let response = client
            .post(&config.api_url)
            .header("Authorization", &config.auth_header)
            .header("Content-Type", content_type)
            .body(json_payload)
            .send()
            .expect("Error sending request");

        println!("Status: {}", response.status());

        if args.delay > 0 {
            sleep(Duration::from_secs(args.delay));
        }
    }
}