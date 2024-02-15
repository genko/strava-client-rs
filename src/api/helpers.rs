//! # helpers.rs
//! Helper Functions for the calling the Strava API

use log::warn;
use reqwest::blocking::{RequestBuilder, Response};
use std::collections::HashMap;

const STRAVA_BASE_URL: &str = "https://www.strava.com/api/v3/";
const API_ERROR_MESSAGE: &str = "API returned an error";

/// ## strava_v3
/// Formats the path for the Strava API endpoint
/// Returns the URL for the Strava API
pub fn strava_v3(path: String) -> String {
    format!("{}{}", STRAVA_BASE_URL, path)
}

/// Sends a request and returns the response.
///
/// # Arguments
///
/// * `request` - The request builder object.
///
/// # Returns
///
/// * `Result<Response, Box<dyn std::error::Error>>` - Result containing the response on success,
/// or an error on failure.
///
/// # Errors
///
/// Returns an error if the request fails or if the response status is a client error.
fn send_request(request: RequestBuilder) -> Result<Response, Box<dyn std::error::Error>> {
    let response = request.send()?;

    if response.status().is_client_error() {
        warn!("API request returned an error: {:?}", response);
        return Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::Other,
            API_ERROR_MESSAGE,
        )));
    }

    Ok(response)
}

/// Sends a GET request to the Strava API using the provided URL and access token.
///
/// # Arguments
///
/// * `url` - The URL to the Strava API endpoint.
/// * `access_token` - The access token for authentication.
///
/// # Returns
///
/// A `Result` containing the response from the Strava API or an error message.
///
/// # Errors
///
/// This function may return an error if there is a problem with the HTTP request or if the server
/// returns an error response.
pub fn fetch_from_strava_api(
    url: String,
    access_token: &str,
) -> Result<Response, Box<dyn std::error::Error>> {
    let client = reqwest::blocking::Client::new();
    let request = client.get(url).bearer_auth(access_token);

    send_request(request)
}

/// Sends a PUT request to the Strava API with the provided URL, access token, and parameters.
///
/// # Arguments
///
/// * `url` - The URL of the Strava API endpoint as a String.
/// * `access_token` - The access token for authentication as a &str.
/// * `params` - A HashMap containing the parameters to be included in the request.
///
/// # Returns
///
/// * `Result<Response, Box<dyn std::error::Error>>` - The result of the request, either a `Response` if successful or an error if not.
pub fn put_to_strava_api(
    url: String,
    access_token: &str,
    params: HashMap<&str, &str>,
) -> Result<Response, Box<dyn std::error::Error>> {
    let client = reqwest::blocking::Client::new();
    let request = client.put(url).bearer_auth(access_token).form(&params);

    send_request(request)
}
