use crate::{models, api};
use log::{info, trace, warn};
use oauth2::AccessToken;

// Get the athlete from the Strava API
// https://developers.strava.com/docs/reference/#api-models-Athlete
// Arguments: access_token: &str
// Returns: JSON object from AtheleteCollection
pub fn get_athlete(access_token: &str) -> Result<models::athlete::AthleteCollection, Box<dyn std::error::Error>> {
    
    let client = reqwest::blocking::Client::new();
    
    // Get the URL for the API
    let url = api::strava_v3("athlete".to_string());
   
    // Call the API with the access token
    let response = client.get(url)
    .bearer_auth(access_token)
    .send()?;

    info!("Calling Athelete API\n");
    
    // Check for errors from the API
    if response.status().is_client_error() {
        warn!("API response: {:?}", response);
        return Err(Box::new(std::io::Error::new(std::io::ErrorKind::Other, "API returned an error")));
    }

    trace!("Athlete API response: {:?}\n", response);
    
    // Parse the JSON response
    let athlete = response.json::<models::athlete::AthleteCollection>()?;
    Ok(athlete)
}

// Get the athlete stats from the Strava API for a specific athlete
// https://developers.strava.com/docs/reference/#api-models-AthleteStats
// Arguments: access_token: &str, athlete_id: &str
// Returns: JSON object from AtheleteStats
pub fn get_athlete_stats(access_token: &str, athlete_id: &str) -> Result<models::athlete::AthleteStats, Box<dyn std::error::Error>> {
    
    let client = reqwest::blocking::Client::new();
    
    // Get the URL for the API
    let path = format!("athletes/{}/stats", athlete_id);
    let url = api::strava_v3(path);
   
    // Call the API with the access token
    let response = client.get(url)
    .bearer_auth(access_token)
    .send()?;

    info!("Calling Athelete Stats API\n");
    
    // Check for errors from the API
    if response.status().is_client_error() {
        warn!("API response: {:?}", response);
        return Err(Box::new(std::io::Error::new(std::io::ErrorKind::Other, "API returned an error")));
    }

    trace!("Athlete Stats API response: {:?}\n", response);
    
    // Parse the JSON response
    let athlete_stats = response.json::<models::athlete::AthleteStats>()?;
    Ok(athlete_stats)
}

pub fn get_athlete_clubs(access_token: &str) -> Result<models::clubs::ClubCollection, Box<dyn std::error::Error>> {
    let url = api::strava_v3("athlete/clubs".to_string());

    let client = reqwest::blocking::Client::new();

    let response = client.get(url)
        .bearer_auth(access_token)
        .send()?;
    let clubs = response.json::<models::clubs::ClubCollection>()?;
    Ok(clubs)
}
