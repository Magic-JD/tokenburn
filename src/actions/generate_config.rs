use std::fs;
use crate::configuration::config::DEFAULT_CONFIG;
use crate::utils::file_system::config_path;

pub fn run() {
    println!("Generating config file...");
    let config_path = config_path();
    if config_path.exists() {
        eprintln!("Config file already exists at {config_path:?}");
    } else if let Err(message) = fs::write(&config_path, DEFAULT_CONFIG)
        .map_err(|e| format!("Error writing config file: {e}"))
    {
        eprintln!("{message}");
    } else {
        println!("Config file generated at {config_path:?}");
    }
}
