//! # Club Module
//!
//! `club.rs` This module is the main interface with the Strava API for club operations. It provides
//! functionality to retrieve data related to a specific club such as the club details,
//! club members, club admins, and club activities.

use crate::api::helpers::{fetch_from_strava_api, fetch_strava_data, strava_v3};
use crate::models::clubs;
use log::{info, trace};

/// Get club details by ID.
///
/// # Arguments
///
/// * `access_token` - The user's access token.
/// * `club_id` - The ID of the club.
///
/// # Returns
///
/// * `Result<clubs::Club>` - Club object or a Boxed Error
pub fn get_club_by_id(
    access_token: &str,
    club_id: &str,
) -> Result<clubs::Club, Box<dyn std::error::Error>> {
    trace!("Club ID: {:?}", club_id);
    info!("Calling Clubs by ID Strava API");
    fetch_strava_data(format!("/clubs/{}", club_id), access_token)
}

/// Get club members by club ID.
///
/// # Arguments
///
/// * `access_token` - The user's access token.
/// * `club_id` - The ID of the club.
///
/// # Returns
///
/// * `Result<clubs::ClubMembers>` - A list of club members or a Boxed Error
pub fn get_club_members(
    access_token: &str,
    club_id: &str,
) -> Result<clubs::ClubMembers, Box<dyn std::error::Error>> {
    info!("Calling Strava Club Members API");
    fetch_strava_data(format!("/clubs/{}/members", club_id), access_token)
}

/// Get club admins by club ID.
///
/// # Arguments
///
/// * `access_token` - The user's access token.
/// * `club_id` - The ID of the club.
///
/// # Returns
///
/// * `Result<clubs::ClubAdmins>` - A list of club admins or a Boxed Error
pub fn get_club_admins(
    access_token: &str,
    club_id: &str,
) -> Result<clubs::ClubAdmins, Box<dyn std::error::Error>> {
    info!("Calling Strava Club Admins API");
    fetch_strava_data(format!("/clubs/{}/admins", club_id), access_token)
}

/// Get club activities by club ID.
///
/// # Arguments
///
/// * `access_token` - The user's access token.
/// * `club_id` - The ID of the club.
///
/// # Returns
///
/// * `Result<clubs::ClubActivities>` - A list of club activities or a Boxed Error
pub fn get_club_activities(
    access_token: &str,
    club_id: &str,
) -> Result<clubs::ClubActivities, Box<dyn std::error::Error>> {
    info!("Calling Strava Get Club Activities API");
    fetch_strava_data(format!("/clubs/{}/activities", club_id), access_token)
}
