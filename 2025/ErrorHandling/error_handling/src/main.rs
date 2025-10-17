/* Error handling recoverable and unrecoverable errors
 * enum Result<T,E>
 * panic!()
 * Error Propagation
 * ? : operator
 *
 * Simulate the above concepts of error handling in Rust using the example:
 * Reads a config file from the disk
 * Parses the contents of it
 * Validates the configuration
 * Propagates errors properly with ?
 * panics in exceptional unrecoverable conditions ( Eg: Critical logic failure )
 */

use std::fs::File;
use std::io::{self, Read};
use std::num::ParseIntError;
use std::path::Path;

// Define a custom error type that can represent various errors
#[derive(Debug)]
enum ConfigError {
    Io(io::Error),
    Parse(ParseIntError),
    MissingField(String),
    InvalidValue(String),
}
//Implement conversion from underlying error type to our custom type
impl From<io::Error> for ConfigError {
    fn from(e: io::Error) -> Self {
        ConfigError::Io(e)
    }
}
impl From<ParseIntError> for ConfigError {
    fn from(e: ParseIntError) -> Self {
        ConfigError::Parse(e)
    }
}

//Struct to hold our configuration
#[derive(Debug)]
struct AppConfig {
    app_name: String,
    max_connections: u32,
}
// Function to read the contents of a file into a string:
fn read_file_to_string(path: &str) -> Result<String, ConfigError> {
    let mut file = std::fs::File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

//parse configuration from the file contents
fn parse_config(contents: &str) -> Result<AppConfig, ConfigError> {
    let mut app_name: Option<String> = None;
    let mut max_connections: Option<u32> = None;

    for line in contents.lines() {
        let line = line.trim();
        if line.starts_with("app_name=") {
            app_name = Some(line["app_name=".len()..].to_string());
        } else if line.starts_with("max_connections=") {
            let val_str = &line["max_connections=".len()..];
            let val = val_str.parse::<u32>()?; // May return ParseIntError
            max_connections = Some(val);
        }
    }
    let app_name = app_name.ok_or(ConfigError::MissingField("app_name".to_string()))?;
    let max_connections =
        max_connections.ok_or(ConfigError::MissingField("max_connections".to_string()))?;

    // Simulate a critical error
    if max_connections == 0 {
        panic!("max_connections cannot be zero — this is a fatal logic error");
    }
    Ok(AppConfig {
        app_name,
        max_connections,
    })
}

fn load_config(path: &str) -> Result<AppConfig, ConfigError> {
    let contents = read_file_to_string(path)?;
    let config = parse_config(&contents)?;
    Ok(config)
}
fn main() {
    let config_path = "config.txt";

    match load_config(config_path) {
        Ok(config) => {
            println!("Configuration loaded Successfully: {:#?}", config);
        }
        Err(e) => {
            eprintln!("Failed to load configuration: {:?}", e);
        }
    }
    println!("Application continues running...");
}
