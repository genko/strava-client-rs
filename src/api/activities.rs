use crate::{models, api};
use log::{info, trace, warn};

pub fn get_activities(access_token: &str) -> Result<models::activities::ActivityCollection, Box<dyn std::error::Error>> {
    
    let client = reqwest::blocking::Client::new();
    
    let url = api::strava_v3("athlete/activities".to_string());
   
    let response = client.get(url)
    .bearer_auth(access_token)
    .send()?;

    info!("Calling Activities API\n");
    
    if response.status().is_client_error() {
        warn!("API response: {:?}", response);
        return Err(Box::new(std::io::Error::new(std::io::ErrorKind::Other, "API returned an error")));
    }

    trace!("Activities API response: {:?}\n", response);
    
    let activities = response.json::<models::activities::ActivityCollection>()?;
    Ok(activities)
}

pub fn get_activities_by_id(access_token: &str, athlete_id: &str) -> Result<models::activities::ActivityCollection, Box<dyn std::error::Error>> {
    
    let client = reqwest::blocking::Client::new();
    
    let path = format!("/activities{}", athlete_id);
    let url = api::strava_v3(path);
   
    let response = client.get(url)
    .bearer_auth(access_token)
    .send()?;

    info!("Calling Activities by ID API\n");
    
    if response.status().is_client_error() {
        warn!("API response: {:?}", response);
        return Err(Box::new(std::io::Error::new(std::io::ErrorKind::Other, "API returned an error")));
    }

    trace!("Activities by ID API response: {:?}\n", response);
    
    let activities = response.json::<models::activities::ActivityCollection>()?;
    Ok(activities)
}