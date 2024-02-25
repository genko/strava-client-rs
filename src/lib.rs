//! # Strava Client
//! A library for the Strava API.
//! # Description
//! This library will authenticate with the Strava API Ouath2 client credentials flow.
//! Store the access token in a file and refresh the token when it expires.
//! In order to use this library you will need to register your application with Strava.
//! Then store the client_id and client_secret as an environment variables.
//! To store the config file outside of the default of config.json, set the environment variable STRAVA_CONFIG_FILE.
//! This library currently supports read operations to the following endpoints:
//! * Athlete
//! * Activities
//! * Clubs
//! * Gear
//!
//! It supports write operations on athlete weight
//!
//! # Examples
//!
//! Get authenticated athlete
//! ```no_run
//! use strava_client_rs::{api::{auth, athlete}};
//! use strava_client_rs::util::auth_config;
//!  use std::env;
//!  use std::path::Path;
//!  
//! fn main() {
//!    //Get the access token from the config file or get a new one
//!    let config_file = env::var("STRAVA_CONFIG_FILE").unwrap_or_else(|_| "config.json".to_string());
//!    let access_token = get_access_token(config_file).unwrap();
//!    let athlete = athlete::get_athlete(access_token.as_str()).unwrap();
//!    println!("Athlete: {:?}\n", athlete);
//!    let athlete_id = athlete.id.to_string();
//! }
//!
//! // Get the access token from the config file or get a new one
//! fn get_access_token(config_file: String) -> Result<String, String> {
//! let client_id =
//!         env::var("STRAVA_CLIENT_ID").expect("Missing the STRAVA_CLIENT_ID environment variable.");
//!     let client_secret = env::var("STRAVA_CLIENT_SECRET")
//!         .expect("Missing the STRAVA_CLIENT_SECRET environment variable.");
//!     let auth_url = "http://www.strava.com/oauth/authorize";
//!     let token_url = "https://www.strava.com/oauth/token";
//!
//!     // Setup default config for auth
//!     let mut config = auth::Config::new(
//!         client_id.to_string(),
//!         client_secret.to_string(),
//!         Default::default(), // no refresh token so set to default which is none
//!         auth_url.to_string(),
//!         token_url.to_string(),
//!     );
//!
//!     // Check if the config file exists and get the access token or get a new one
//!     if Path::new(&config_file).exists() {
//!         config.refresh_token = Some(auth_config::config_file::load_config().refresh_token);
//!         let refresh_access_token = auth::get_refresh_token(config);
//!         Ok(refresh_access_token.unwrap().to_string())
//!     } else {
//!         let access_token = auth::get_authorization(config);
//!         Ok(access_token.unwrap().to_string())
//!     }
//! }
//! ```
//! Disclaimer: This library is not affiliated with Strava and is not an official Strava library.

pub mod api;
pub mod models;
pub mod util;
