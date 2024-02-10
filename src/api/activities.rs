use log::{info, trace};

use crate::api::helpers::{fetch_from_strava_api, strava_v3};
use crate::models::activities;

// Get the activities from the Strava API for logged in athlete
// https://developers.strava.com/docs/reference/#api-Activities
// Arguments: access_token: &str
// Returns: JSON object from ActivityCollection
pub fn get_activities(
    access_token: &str,
) -> Result<activities::ActivityCollection, Box<dyn std::error::Error>> {
    let url = strava_v3("athlete/activities".to_string());
    info!("Calling Strava Activities API\n");

    let response = fetch_from_strava_api(url, access_token)?;
    trace!("Activities API response: {:?}\n", response);

    let activities: activities::ActivityCollection = response.json()?;
    Ok(activities)
}

// Get activity by ID
// Arguments: access_token: &str, activity_id: &str
// Returns json object from ActivityElement model
pub fn get_activities_by_id(
    access_token: &str,
    activity_id: &str,
) -> Result<activities::ActivityElement, Box<dyn std::error::Error>> {
    let url = strava_v3(format!("/activities/{}", activity_id));
    let response = fetch_from_strava_api(url, access_token)?;

    info!("Calling Activities by ID API\n");

    trace!("Activities by ID API response: {:?}\n", response);

    let activity: activities::ActivityElement = response.json()?;
    Ok(activity)
}
