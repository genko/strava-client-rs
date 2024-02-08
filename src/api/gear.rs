use crate::{models, api};
use log::{info, trace, warn};

pub fn get_gear(access_token: &str, gear_id: &str) -> Result<models::gear::GearCollection, Box<dyn std::error::Error>> {
    let client = reqwest::blocking::Client::new();
    trace!("Gear ID {}\n", gear_id);
    let path = format!("gear/{}", gear_id);
    let url = api::strava_v3(path);

    let response = client.get(url).
        bearer_auth(access_token).
        send()?;

    info!("Calling Gear Stats API\n");

    if response.status().is_client_error() {
        warn!("API response: {:?}", response);
        return Err(Box::new(std::io::Error::new(std::io::ErrorKind::Other, "API returned an error")));
    }
    trace!("Athlete Stats API response: {:?}\n", response);

    let gear_info = response.json::<models::gear::GearCollection>()?;
    Ok(gear_info)
}