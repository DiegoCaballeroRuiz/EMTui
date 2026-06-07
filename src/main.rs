use std::{env, process::exit};

mod flags;
use flags::Flags;

mod credentials;
use credentials::{Credentials, get_credentials};

fn main() {
    let args: Vec<String> = env::args().collect();
    let flags = Flags::parse(args);

    match flags {
        Ok(flags) => run(flags),
        Err(msg) => {
            if msg == "Usage" {
                print_help_message();
                exit(0);
            } else {
                eprintln!("ERROR: {}", msg);
                exit(1);
            }
        }
    }
}

fn run(flags: Flags) {
    let credentials = match get_credentials() {
        Ok(credentials) => credentials,
        Err(e) => {
            eprintln!("ERROR: {}", e);
            exit(1);
        }
    };

    let token = match login(credentials) {
        Ok(t) => t,
        Err(e) => {
            eprintln!("ERROR: {}", e);
            exit(1);
        }
    };

    let arrivals = match get_arrivals(token, flags) {
        Ok(arrivals) => arrivals,
        Err(e) => {
            eprintln!("ERROR: {}", e);
            exit(1);
        }
    };

    match display(&arrivals) {
        Ok(_) => (),
        Err(e) => {
            eprintln!("ERROR: {}", e);
            exit(1);
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
        .ok_or("Token not found in response".to_string())?
        .to_string();

    // Return token
    Ok(token)
}

fn get_arrivals(token: String, flags: Flags) -> Result<serde_json::Value, String> {
    // Create a client
    let client = reqwest::blocking::Client::new();

    // Calculate the url to get the info
    let stop_id = flags.stop_code;
    let url = match flags.bus_filter {
        Some(line) => format!(
            "https://openapi.emtmadrid.es/v2/transport/busemtmad/stops/{stop_id}/arrives/{line}/"
        ),
        None => {
            format!("https://openapi.emtmadrid.es/v2/transport/busemtmad/stops/{stop_id}/arrives/")
        }
    };

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

fn display(arrivals: &serde_json::Value) -> Result<(), String> {
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
                .ok_or("estimateArrive missing or not a number".to_string())?;

            if seconds_remaining < 60 {
                println!("Line {:<5} {:<25} <1m", line, destination);
            } else {
                let minutes_remaining = seconds_remaining / 60;
                println!(
                    "Line {:<5} {:<25} {}m",
                    line, destination, minutes_remaining
                );
            }
        }
    }
    else {
        println!("No arrivals expected");
    }

    Ok(())
}

fn print_help_message() {
    print!(
        "Usage:
    emtui <bus-stop>: For info on all buses of a stop
    emtui <bus-stop> -B <bus-number>: For the two next appereances of a bus in a stop"
    );
}
