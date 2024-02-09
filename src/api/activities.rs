use crate::{models, api};
use log::{info, trace, warn};
use crate::api::handle_api_error;

// Get the activities from the Strava API for logged in athlete
// https://developers.strava.com/docs/reference/#api-Activities
// Arguments: access_token: &str
// Returns: JSON object from ActivityCollection
pub fn get_activities(access_token: &str) -> Result<models::activities::ActivityCollection, Box<dyn std::error::Error>> {
    
    let client = reqwest::blocking::Client::new();
    
    let url = api::strava_v3("athlete/activities".to_string());
   
    let response = client.get(url)
    .bearer_auth(access_token)
    .send()?;

    info!("Calling Activities API\n");

    handle_api_error(response.status())?;

    trace!("Activities API response: {:?}\n", response);
    
    let activities = response.json::<models::activities::ActivityCollection>()?;
    Ok(activities)
}

// Get activity by ID
// Arguments: access_token: &str, activity_id: &str
// Returns json object from ActivityElement model
pub fn get_activities_by_id(access_token: &str, activity_id: &str) -> Result<models::activities::ActivityElement, Box<dyn std::error::Error>> {
    
    let client = reqwest::blocking::Client::new();
    
    let path = format!("/activities/{}", activity_id);
    let url = api::strava_v3(path);
   
    let response = client.get(url)
    .bearer_auth(access_token)
    .send()?;

    info!("Calling Activities by ID API\n");

    handle_api_error(response.status())?;
    trace!("Activities by ID API response: {:?}\n", response);

    let activity = response.json::<models::activities::ActivityElement>()?;
    Ok(activity)
}