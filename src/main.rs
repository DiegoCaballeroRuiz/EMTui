use std::{env, process::exit};

mod flags;
use flags::Flags;

mod config;
use config::{Credentials, get_credentials};

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
}

fn login(credentials: Credentials) -> Result<String, String> {
    // Create a client (like opening a browser)
    let client = reqwest::blocking::Client::new();

    // Build and send the request
    let response = client
        .post("https://openapi.emtmadrid.es/v1/mobilitylabs/user/login/")
        .header("email", credentials.email)
        .header("password", credentials.password)
        .send()
        .map_err(|e| e.to_string())?;

    // Parse the response body as JSON
    let json: serde_json::Value = response.json().map_err(|e| e.to_string())?;

    // Dig out the token
    let token = json["data"][0]["token"]
        .as_str()
        .ok_or("Token not found in response".to_string())?
        .to_string();

    // Return token
    Ok(token)
}

fn print_help_message() {
    print!(
        "Usage:
    emtui <bus-stop>: For info on all buses of a stop
    emtui <bus-stop> -B <bus-number>: For the two next appereances of a bus in a stop"
    );
}
