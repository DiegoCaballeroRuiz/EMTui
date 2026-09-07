pub struct Credentials {
    pub email: String,
    pub password: String,
}

pub fn get_credentials() -> Result<Credentials, String> {
    if let Ok(credentials) = load_config() {
        Ok(credentials)
    } else {
        let credentials = prompt_credentials()?;
        save_config(&credentials)?;
        Ok(credentials)
    }
}

fn load_config() -> Result<Credentials, String> {
    // Locate path of config file
    let config_path = dirs::config_dir()
        .ok_or("Could not find config directory")?
        .join("emtui")
        .join("credentials.txt");

    // Parse content into email and password
    let content = match std::fs::read_to_string(&config_path) {
        Ok(c) => c,
        Err(e) => return Err(e.to_string()),
    };
    let mut lines = content.lines();
    let email = lines.next().ok_or("Missing email in config file")?;
    let password = lines.next().ok_or("Missing password in config file")?;

    // Return struct
    let email = email.to_owned();
    let password = password.to_owned();
    Ok(Credentials { email, password })
}

fn prompt_credentials() -> Result<Credentials, String> {
    // Prompt for email
    println!("Enter your EMT email:");
    let mut email = String::new();
    std::io::stdin()
        .read_line(&mut email)
        .map_err(|e| e.to_string())?;

    // Prompt for password
    println!("Enter your EMT password:");
    let mut password = String::new();
    std::io::stdin()
        .read_line(&mut password)
        .map_err(|e| e.to_string())?;

    // Return struct
    Ok(Credentials { email, password })
}

fn save_config(credentials: &Credentials) -> Result<(), String> {
    // Locate path of config file
    let config_path = dirs::config_dir()
        .ok_or("Could not find config directory")?
        .join("emtui")
        .join("credentials.txt");

    // Create directory in case it doesent exist
    if let Some(dir) = config_path.parent() {
        std::fs::create_dir(dir).map_err(|e| e.to_string())?;
    } else {
        return Err("Couldn't find config path".to_string());
    }

    // Create and write file with config
    std::fs::write(
        config_path,
        format!(
            "{}\n{}",
            credentials.email.trim(),
            credentials.password.trim()
        ),
    )
    .map_err(|e| e.to_string())?;

    // Confirm that credentials were saved
    println!("Credentials saved!");
    Ok(())
}
