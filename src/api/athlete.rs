//! # Module athlete
//! Get the athlete from the Strava API
//! https://developers.strava.com/docs/reference/#api-models-Athlete
//! Arguments: access_token: &str
//! Returns: JSON object from AthleteCollection

use crate::api::helpers::{fetch_from_strava_api, put_to_strava_api, strava_v3};
use crate::models::{athlete, clubs};
use log::{info, trace};
use reqwest::blocking::Response;
use serde_json::Value::String;
use std::collections::HashMap;
use std::error::Error;

/// Public function to get the athlete from the Strava API
/// Arguments: access_token: &str
/// Returns: JSON object from AthleteCollection
/// Example: let athlete = get_athlete("access_token");
pub fn get_athlete(
    access_token: &str,
) -> Result<athlete::AthleteCollection, Box<dyn std::error::Error>> {
    // Get the URL for the API
    let url = strava_v3("athlete".to_string());
    info!("Calling Strava Athlete API");
    // Call the API with the access token
    let response = fetch_from_strava_api(url, access_token)?;

    trace!("Athlete API response: {:?}\n", response);

    // Parse the JSON response
    let athlete: athlete::AthleteCollection = response.json()?;
    Ok(athlete)
}

/// Get the athlete stats from the Strava API for a specific athlete
/// https://developers.strava.com/docs/reference/#api-models-AthleteStats
/// Arguments: access_token: &str, athlete_id: &str
/// Returns: JSON object from AthleteStats
/// Example: let stats = get_athlete_stats("access_token", "athlete_id");
pub fn get_athlete_stats(
    access_token: &str,
    athlete_id: &str,
) -> Result<athlete::AthleteStats, Box<dyn std::error::Error>> {
    // Get the URL for the API
    let path = format!("athletes/{}/stats", athlete_id);
    let url = strava_v3(path);

    let response = fetch_from_strava_api(url, access_token)?;
    info!("Calling Athlete Stats API\n");

    trace!("Athlete Stats API response: {:?}\n", response);

    // Parse the JSON response
    let athlete_stats: athlete::AthleteStats = response.json()?;
    Ok(athlete_stats)
}

/// Get the athlete clubs from the Strava API for a specific athlete
/// Arguments: access_token: &str
/// Returns: JSON object from ClubCollection
/// Example: let clubs = get_athlete_clubs("access_token");
pub fn get_athlete_clubs(
    access_token: &str,
) -> Result<clubs::ClubCollection, Box<dyn std::error::Error>> {
    let url = strava_v3("athlete/clubs".to_string());

    let response = fetch_from_strava_api(url, access_token)?;
    let clubs: clubs::ClubCollection = response.json()?;
    Ok(clubs)
}

/// Update the weight of the logged-in athlete.
///
/// # Arguments
///
/// * `access_token` - The access token of the logged-in athlete.
/// * `weight` - The new weight to update.
///
/// # Returns
///
/// * A `Result` containing the response from the API call, or an error if any occurred.
pub fn update_athlete_weight(access_token: &str, weight: &str) -> Result<Response, Box<dyn Error>> {
    let url = strava_v3("athlete".to_string());
    let mut params = HashMap::new();
    params.insert("weight", weight);

    info!("Calling Athlete Update Weight API\n");
    put_to_strava_api(url, access_token, params)
}
