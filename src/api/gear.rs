//! # Gear Module
//! This module is responsible for retrieving gear information from an API.
//! It makes use of the gear id and access token to pull the info from Strava Gear API

use crate::api::helpers::fetch_strava_data;
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
    info!("Calling Strava Gear API");
    fetch_strava_data(format!("gear/{}", gear_id), access_token)
}
