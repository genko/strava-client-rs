//! # Gear Module
//! This module is responsible for retrieving gear information from an API.
//! It makes use of the gear id and access token to pull the info from Strava Gear API

use crate::api::helpers::{fetch_from_strava_api, strava_v3};
use crate::models::gear::GearCollection;
use log::{info, trace};

/// Function get_gear
/// Arguments: access_token: &str, gear_id: &str
/// Returns gear object
pub fn get_gear(
    access_token: &str,
    gear_id: &str,
) -> Result<GearCollection, Box<dyn std::error::Error>> {
    trace!("Gear ID {}\n", gear_id);
    let url = strava_v3(format!("gear/{}", gear_id));
    info!("Calling Strava Gear API");
    let response = fetch_from_strava_api(url, access_token)?;
    let gear_info: GearCollection = response.json()?;
    Ok(gear_info)
}
