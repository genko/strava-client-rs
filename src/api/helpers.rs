//! # helpers.rs
//! Helper Functions for the calling the Strava API

use log::warn;
use reqwest::blocking::Response;

const STRAVA_BASE_URL: &str = "https://www.strava.com/api/v3/";
const API_ERROR_MESSAGE: &str = "API returned an error";

/// ## strava_v3
/// Formats the path for the Strava API endpoint
/// Returns the URL for the Strava API
pub fn strava_v3(path: String) -> String {
    format!("{}{}", STRAVA_BASE_URL, path)
}

/// ## fetch_from_strava_api
/// Takes the URL and access token to perform a *GET* request
/// for the giving API endpoint in the url
/// Returns *reqwest response* or error message from the response
pub fn fetch_from_strava_api(
    url: String,
    access_token: &str,
) -> Result<Response, Box<dyn std::error::Error>> {
    let client = reqwest::blocking::Client::new();
    let response = client.get(url).bearer_auth(access_token).send()?;

    if response.status().is_client_error() {
        warn!("API request returned an error: {:?}", response);
        return Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::Other,
            API_ERROR_MESSAGE,
        )));
    }
    Ok(response)
}
