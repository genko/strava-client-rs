//! # Module activities
//! This module provides a set of functions to interact with the Strava API for activities.
//! https://developers.strava.com/docs/reference/#api-Activities

use crate::api::helpers::fetch_strava_data;
use crate::api::strava_v3;
use crate::models::activities;
use log::{info, trace};
use std::collections::HashMap;

/// Struct representing parameters for activities
#[derive(Debug)]
pub struct ActivitiesParams {
    pub access_token: String,
    pub before: Option<String>,
    pub after: Option<String>,
    pub per_page: Option<String>,
    pub page: Option<String>,
}

/// Takes an `ActivitiesParams` object and returns a `HashMap` of parameter names and their corresponding values.
///
/// # Arguments
///
/// * `params` - A reference to an `ActivitiesParams` object.
///
/// # Returns
///
/// A `HashMap<&str, String>` containing the parameter names and their corresponding values.
fn prepare_params(params: &ActivitiesParams) -> HashMap<&str, String> {
    let mut map_params = HashMap::new();
    map_params.insert("access_token", params.access_token.clone());
    if let Some(before) = &params.before {
        map_params.insert("before", before.clone());
    }
    if let Some(after) = &params.after {
        map_params.insert("after", after.clone());
    }
    if let Some(per_page) = &params.per_page {
        map_params.insert("per_page", per_page.clone());
    }
    if let Some(page) = &params.page {
        map_params.insert("page", page.clone());
    }
    map_params
}

/// Get the activities from the Strava API for logged in athlete
/// https://developers.strava.com/docs/reference/#api-Activities
/// Arguments: access_token: &str
/// Returns: JSON object from ActivityCollection
pub fn get_activities(
    params: &ActivitiesParams,
) -> Result<activities::ActivityCollection, Box<dyn std::error::Error>> {
    let map_params = prepare_params(params);
    trace!("Athlete API parameters: {:?}", map_params);
    // This is using a very specific call to the strava API passing in optional params
    // the standard fetch_strava_data helper would not work here so calling the request manually
    let client = reqwest::blocking::Client::new();
    let url = strava_v3("athlete/activities".to_string());
    let response = client.get(url).query(&map_params).send()?;
    let json: activities::ActivityCollection = response.json()?;
    Ok(json)
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
