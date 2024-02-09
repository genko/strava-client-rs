use crate::api::{handle_api_error, strava_v3};
use crate::models;

pub fn get_club_by_id(access_token: &str, club_id: &str) -> Result<models::clubs::Club, Box<dyn std::error::Error>> {
    let path = format!("/clubs/{}", club_id);
    let url = strava_v3(path);

    let client = reqwest::blocking::Client::new();

    let response = client.get(url).
        bearer_auth(access_token).
        send()?;
    handle_api_error(response.status())?;
    let club: models::clubs::Club = response.json()?;
    Ok(club)
}

pub fn get_club_members(access_token: &str, club_id: &str) -> Result<models::clubs::ClubMembers, Box<dyn std::error::Error>>  {

    let path = format!("/clubs/{}/members", club_id);
    let url = strava_v3(path);

    let client = reqwest::blocking::Client::new();

    let response = client.get(url).bearer_auth(access_token).send()?;

    handle_api_error(response.status())?;
    let members: models::clubs::ClubMembers = response.json()?;
    Ok(members)
}

pub fn get_club_admins(access_token: &str, club_id: &str) -> Result<models::clubs::ClubAdmins, Box<dyn std::error::Error>>  {

    let path = format!("/clubs/{}/admins", club_id);
    let url = strava_v3(path);

    let client = reqwest::blocking::Client::new();

    let response = client.get(url).bearer_auth(access_token).send()?;

    handle_api_error(response.status())?;
    let admins: models::clubs::ClubAdmins = response.json()?;
    Ok(admins)
}

pub fn get_club_activities(access_token: &str, club_id: &str) -> Result<models::clubs::ClubActivities, Box<dyn std::error::Error>>  {

    let path = format!("/clubs/{}/activities", club_id);
    let url = strava_v3(path);

    let client = reqwest::blocking::Client::new();

    let response = client.get(url).bearer_auth(access_token).send()?;

    handle_api_error(response.status())?;
    let club_activities: models::clubs::ClubActivities = response.json()?;
    Ok(club_activities)
}