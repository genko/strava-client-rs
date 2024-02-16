//! # Module athlete
//! Get the athlete from the Strava API
//! https://developers.strava.com/docs/reference/#api-models-Athlete
//! Arguments: access_token: &str
//! Returns: JSON object from AthleteCollection

use crate::api::helpers::{fetch_strava_data, put_to_strava_api, strava_v3};
use crate::models::{athlete, clubs};
use log::{info};
use reqwest::blocking::Response;

use std::collections::HashMap;
use std::error::Error;

/// Public function to get the athlete from the Strava API
/// Arguments: access_token: &str
/// Returns: JSON object from AthleteCollection
/// Example: let athlete = get_athlete("access_token");
pub fn get_athlete(
    access_token: &str,
) -> Result<athlete::AthleteCollection, Box<dyn std::error::Error>> {
    info!("Calling Strava Athlete API");
    // Call the API with the access token
    fetch_strava_data("athlete".to_string(), access_token)
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
    info!("Calling Athlete Stats API\n");
    fetch_strava_data(format!("athletes/{}/stats", athlete_id), access_token)
}

/// Get the athlete clubs from the Strava API for a specific athlete
/// Arguments: access_token: &str
/// Returns: JSON object from ClubCollection
/// Example: let clubs = get_athlete_clubs("access_token");
pub fn get_athlete_clubs(
    access_token: &str,
) -> Result<clubs::ClubCollection, Box<dyn std::error::Error>> {
    info!("Calling Athlete Clubs API\n");
    fetch_strava_data("athlete/clubs".to_string(), access_token)
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
