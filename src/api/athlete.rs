use log::{info, trace, warn};
use crate::api::helpers::{fetch_from_strava_api, strava_v3};
use crate::models::{athlete, clubs};

// Get the athlete from the Strava API
// https://developers.strava.com/docs/reference/#api-models-Athlete
// Arguments: access_token: &str
// Returns: JSON object from AtheleteCollection
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

// Get the athlete stats from the Strava API for a specific athlete
// https://developers.strava.com/docs/reference/#api-models-AthleteStats
// Arguments: access_token: &str, athlete_id: &str
// Returns: JSON object from AthleteStats
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

pub fn get_athlete_clubs(
    access_token: &str,
) -> Result<clubs::ClubCollection, Box<dyn std::error::Error>> {
    let url = strava_v3("athlete/clubs".to_string());

    let response = fetch_from_strava_api(url, access_token)?;
    let clubs: clubs::ClubCollection = response.json()?;
    Ok(clubs)
}
