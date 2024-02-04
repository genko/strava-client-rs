use crate::{models, api};
use log::{info, trace, warn};

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