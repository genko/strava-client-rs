/// # auth_config.rs
/// Checks to see if a config file exists and pulls the access_code and refresh_token
/// Can create a config file that stores the access_code and refresh_token if not found
/// # Example
/// ```no_run
///  // Check if the config file exists and get the access token or get a new one
///  if Path::new(&config_file).exists() {
///     config.refresh_token = Some(auth_config::config_file::load_config().refresh_token);
///     let refresh_access_token = auth::get_refresh_token(config);
///     Ok(refresh_access_token.unwrap().to_string())
///  } else {
///      let access_token = auth::get_authorization(config);
///     Ok(access_token.unwrap().to_string())
///  }
/// ```
use log::{trace, warn};
use serde::{Deserialize, Serialize};
use std::env;
use std::fs::File;

/// ## config_file module to create a config file or retrieve the access_token, refresh_token
pub mod config_file {

    use super::*;

    /// ConfigFile struct to hold the access token and refresh token
    #[derive(Debug, Serialize, Deserialize)]
    pub struct ConfigFile {
        pub access_token: String,
        pub refresh_token: String,
    }

    /// Private function to get the config file from the environment variable STRAVA_CONFIG_FILE or set it to config.json
    fn get_config_file() -> String {
        let config_file =
            env::var("STRAVA_CONFIG_FILE").unwrap_or_else(|_| "config.json".to_string());
        trace!("Using config file: {}", config_file);
        config_file
    }

    /// Write the access token and refresh token to the config file
    pub fn write_config(access_token: &String, refresh_token: &String) {
        let config = ConfigFile {
            access_token: access_token.to_string(),
            refresh_token: refresh_token.to_string(),
        };

        let get_file = get_config_file();

        let config_file = File::create(get_file).unwrap_or_else(|e| {
            warn!("Couldn't create file: {}", e);
            panic!("Couldn't create file: {}", e);
        });
        serde_json::to_writer_pretty(config_file, &config).unwrap_or_else(|e| {
            warn!("Couldn't write to file: {}", e);
            panic!("Couldn't write to file: {}", e);
        });
    }

    /// Load the config file from JSON and return a Config struct.
    /// Returns a Config struct or panics if it can't open or read the file.
    pub fn load_config() -> ConfigFile {
        let get_file = get_config_file();

        let config_file = std::fs::File::open(get_file).unwrap_or_else(|e| {
            warn!("Couldn't open file: {}", e);
            panic!("Couldn't open file: {}", e);
        });

        serde_json::from_reader(config_file).unwrap_or_else(|e| {
            panic!("Couldn't read file: {}", e);
        })
    }
}
