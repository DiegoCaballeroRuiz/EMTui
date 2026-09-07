use std::env;

mod flags;
use flags::Flags;

mod credentials;
use credentials::{Credentials, get_credentials};

use crate::flags::ParseFlagsError;

fn main() {
    let flags = Flags::parse(env::args());

    match flags {
        Ok(flags) => run(flags),
        Err(err_type) => match err_type {
            ParseFlagsError::Usage => print_help_message(),
            ParseFlagsError::UnparseableStopNumber => eprintln!("Stop code must be a number"),
            ParseFlagsError::UnkownFlag(flag) => eprintln!("Unknown flag \'{flag}\'"),
            ParseFlagsError::BusWithoutNumber => {
                eprintln!("-B or --bus flag requires a bus name argument");
            }
        },
    }
}

fn run(flags: Flags) {
    let credentials = match get_credentials() {
        Ok(credentials) => credentials,
        Err(e) => {
            eprintln!("ERROR: {e}");
            return;
        }
    };

    let token = match login(credentials) {
        Ok(t) => t,
        Err(e) => {
            eprintln!("ERROR: {e}");
            return;
        }
    };

    let result = match flags {
        Flags::Display {
            stop_code,
            bus_filter,
        } => display(token, &stop_code, bus_filter),
        Flags::List { search_pattern } => list(token, search_pattern),
    };

    if let Err(e) = result {
        eprintln!("ERROR: {e}");
    }
}

fn display(token: String, stop_id: &str, bus_filter: Option<String>) -> Result<(), String> {
    let arrivals = get_arrivals(token, stop_id, bus_filter)?;
    display_arrivals(&arrivals)
}

fn list(token: String, search_pattern: Option<String>) -> Result<(), String> {
    let stops = get_stops(token)?;
    list_stops(&stops, search_pattern);

    Ok(())
}

fn get_stops(token: String) -> Result<serde_json::Value, String> {
    // Get url
    const URL: &str = "https://openapi.emtmadrid.es/v1/transport/busemtmad/stops/list/";

    // Create a client
    let client = reqwest::blocking::Client::new();

    // Build and send the request
    let response = client
        .post(URL)
        .header("accessToken", token)
        .send()
        .map_err(|e| e.to_string())?;

    // Parse the response body as JSON
    let stops: serde_json::Value = response.json().map_err(|e| e.to_string())?;

    // Return json
    Ok(stops)
}

fn list_stops(stops: &serde_json::Value, search_pattern: Option<String>) {
    // Get pattern to match
    let pattern = search_pattern.unwrap_or_default().to_lowercase();

    // Print stops that match the pattern
    if let Some(data) = stops["data"].as_array() {
        for stop in data {
            if let (Some(name), Some(code)) = (stop["name"].as_str(), stop["node"].as_str()) {
                let pattern_is_contained = name.to_lowercase().contains(&pattern);
                if pattern_is_contained {
                    println!("{name:<45}-> {code:<5}");
                }
            }
        }
    }
}

fn login(credentials: Credentials) -> Result<String, String> {
    // Create a client
    let client = reqwest::blocking::Client::new();

    // Build and send the request
    let response = client
        .get("https://openapi.emtmadrid.es/v2/mobilitylabs/user/login/")
        .header("email", credentials.email)
        .header("password", credentials.password)
        .send()
        .map_err(|e| e.to_string())?;

    // Parse the response body as JSON
    let json: serde_json::Value = response.json().map_err(|e| e.to_string())?;

    // Dig out the token
    let token = json["data"][0]["accessToken"]
        .as_str()
        .ok_or_else(|| "Token not found in response".to_string())?
        .to_string();

    // Return token
    Ok(token)
}

fn get_arrivals(
    token: String,
    stop_id: &str,
    bus_filter: Option<String>,
) -> Result<serde_json::Value, String> {
    // Create a client
    let client = reqwest::blocking::Client::new();

    // Calculate the url to get the info
    let url = bus_filter.map_or_else(|| format!("https://openapi.emtmadrid.es/v2/transport/busemtmad/stops/{stop_id}/arrives/"), |line| format!(                                       
        "https://openapi.emtmadrid.es/v2/transport/busemtmad/stops/{stop_id}/arrives/{line}/"                                                                                      
    ));

    // Build and send the request
    let response = client
        .post(url)
        .header("accessToken", token)
        .json(&serde_json::json!({
            "cultureInfo": "EN",
            "Text_StopRequired_YN": "Y",
            "Text_EstimationsRequired_YN": "Y",
            "Text_IncidencesRequired_YN": "N"
        }))
        .send()
        .map_err(|e| e.to_string())?;

    // Parse the response body as JSON
    let arrivals: serde_json::Value = response.json().map_err(|e| e.to_string())?;

    // Return json
    Ok(arrivals)
}

fn display_arrivals(arrivals: &serde_json::Value) -> Result<(), String> {
    // Print stop name and separator
    if let Some(stop_name) = arrivals["data"][0]["StopInfo"][0]["stopName"].as_str() {
        println!("{stop_name}");
        println!("{}", "─".repeat(40));
    } else {
        return Err("Stop does not exist".to_owned());
    }

    // Print every bus in the data array
    if let Some(buses) = arrivals["data"][0]["Arrive"].as_array() {
        if buses.is_empty() {
            println!("No buses found for the requested line at this stop.");
            return Ok(());
        }

        for bus in buses {
            let line = bus["line"].as_str().unwrap_or("?");
            let destination = bus["destination"].as_str().unwrap_or("?");

            let seconds_remaining = bus["estimateArrive"]
                .as_u64()
                .ok_or_else(|| "estimateArrive missing or not a number".to_string())?;

            if seconds_remaining < 60 {
                println!("Line {line:<5} {destination:<25} <1m");
            } else {
                let minutes_remaining = seconds_remaining / 60;
                println!("Line {line:<5} {destination:<25} {minutes_remaining}m");
            }
        }
    } else {
        println!("No arrivals expected");
    }

    Ok(())
}

fn print_help_message() {
    print!(
        "Usage:
    emtui <bus-stop>: For info on all buses of a stop
    emtui <bus-stop> -B <bus-number>: For the two next appereances of a bus in a stop
    emtui -l/--list (pattern): For a list of bus stops containing said pattern as part of the name
                               leave the pattern empty for a list of all stops"
    );
}
