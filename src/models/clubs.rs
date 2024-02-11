/// #  Clubs Model
/// This model is used to represent the club object returned by the Strava API
/// Documentation: https://developers.strava.com/docs/reference/#api-Clubs
use serde::{Deserialize, Serialize};

/// Collection of clubs
pub type ClubCollection = Vec<Club>;

/// Club fields returned from the API
#[derive(Debug, Serialize, Deserialize)]
pub struct Club {
    pub id: i64,
    pub resource_state: i64,
    pub name: String,
    pub profile_medium: String,
    pub profile: String,
    pub cover_photo: String,
    pub cover_photo_small: String,
    pub activity_types: Vec<Option<serde_json::Value>>,
    pub activity_types_icon: String,
    pub dimensions: Vec<String>,
    pub sport_type: String,
    pub localized_sport_type: String,
    pub city: String,
    pub state: String,
    pub country: String,
    pub private: bool,
    pub member_count: i64,
    pub featured: bool,
    pub verified: bool,
    pub url: String,
}

/// Collection of members of the club
pub type ClubMembers = Vec<Member>;

/// Club member fields
#[derive(Debug, Serialize, Deserialize)]
pub struct Member {
    pub resource_state: i64,
    pub firstname: String,
    pub lastname: String,
    pub membership: String,
    pub admin: bool,
    pub owner: bool,
}

/// Collection of club admins
pub type ClubAdmins = Vec<Admin>;

/// Club admin fields
#[derive(Debug, Serialize, Deserialize)]
pub struct Admin {
    pub resource_state: i64,
    pub firstname: String,
    pub lastname: String,
}

/// Collection of activities of a club
pub type ClubActivities = Vec<ClubActivity>;

/// Club activity fields
#[derive(Debug, Serialize, Deserialize)]
pub struct ClubActivity {
    pub resource_state: i64,
    pub athlete: Athlete,
    pub name: String,
    pub distance: f64,
    pub moving_time: i64,
    pub elapsed_time: i64,
    pub total_elevation_gain: f64,
    #[serde(rename = "type")]
    pub activity_type: String,
    pub sport_type: String,
    pub workout_type: Option<serde_json::Value>,
}

/// Athlete details of the club activity
#[derive(Debug, Serialize, Deserialize)]
pub struct Athlete {
    pub resource_state: i64,
    pub firstname: String,
    pub lastname: String,
}
