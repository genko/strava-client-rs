//! # Module activities
//! This module provides a set of functions to interact with the Strava API for activities.
//! https://developers.strava.com/docs/reference/#api-Activities

use crate::api::helpers::{fetch_from_strava_api, fetch_strava_data, strava_v3};
use crate::models::activities;
use log::{info, trace};

/// Get the activities from the Strava API for logged in athlete
/// https://developers.strava.com/docs/reference/#api-Activities
/// Arguments: access_token: &str
/// Returns: JSON object from ActivityCollection
pub fn get_activities(
    access_token: &str,
) -> Result<activities::ActivityCollection, Box<dyn std::error::Error>> {
    info!("Calling Strava Activities API\n");
    fetch_strava_data("athlete/activities".to_string(), access_token)
}

/// Get activity by ID
/// Arguments: access_token: &str, activity_id: &str
/// Returns json object from ActivityElement model
pub fn get_activities_by_id(
    access_token: &str,
    activity_id: &str,
) -> Result<activities::ActivityElement, Box<dyn std::error::Error>> {
    info!("Calling Activities by ID API\n");
    fetch_strava_data(format!("/activities/{}", activity_id), access_token)
}
