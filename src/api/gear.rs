// Gear Module
// This module is responsible for retrieving gear information from an API.
// It makes use of the gear id and access token to pull the info from Strava Gear API
use crate::{api, models};
use log::{info, trace, warn};
use crate::api::handle_api_error;

// Function get_gear
// Arguments: access_token: &str, gear_id: &str
// Returns gear object
pub fn get_gear(
    access_token: &str,
    gear_id: &str,
) -> Result<models::gear::GearCollection, Box<dyn std::error::Error>> {
    let client = reqwest::blocking::Client::new();
    trace!("Gear ID {}\n", gear_id);
    let path = format!("gear/{}", gear_id);
    let url = api::strava_v3(path);

    let response = client.get(url).bearer_auth(access_token).send()?;

    info!("Calling Gear Stats API\n");

    handle_api_error(response.status())?;

    trace!("Gear API response: {:?}\n", response);

    let gear_info = response.json::<models::gear::GearCollection>()?;
    Ok(gear_info)
}
