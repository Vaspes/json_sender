# JSON Sender

A Rust application (written by AI model as an example app) to read a JSON payloads from a file and send each as a POST request with a configurable delay.

## Features

* Reads a JSON file containing an list of payloads (one per file).
* Sends each JSON object as the body of an HTTP POST request.
* Supports the  "application/infracht.load.v1+json;charset=UTF-8" content type.
* Configurable API address and authentication header via a TOML configuration file.
* Optional delay between requests.

## Requirements

* Rust (installation instructions: https://www.rust-lang.org/tools/install)

## Building
````bash
cd json_sender
cargo build --release
````
## Usage
1. Create config file:
````Ini,Toml
    api_url = "https://your-api-endpoint"
    auth_header = "YourActualAuthHeaderString"
````
2. Prepare a JSON data file
* Each line of the file should contain a valid JSON object.
