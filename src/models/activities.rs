//! # Activities model
//! This model is used to represent the athlete object returned by the Strava API
//! Documentation: https://developers.strava.com/docs/reference/#api-Activities
//! Contains helper methods to convert units
use serde::{Deserialize, Serialize};

/// Collection used to retrieve all activities
pub type ActivityCollection = Vec<ActivityElement>;

/// Single Activity Strut
#[derive(Serialize, Deserialize, Debug)]
pub struct ActivityElement {
    pub resource_state: i64,
    pub athlete: Athlete,
    pub name: String,
    pub distance: f64,
    pub moving_time: i64,
    pub elapsed_time: i64,
    pub total_elevation_gain: f64,
    pub sport_type: ActivityType,
    pub workout_type: Option<serde_json::Value>,
    pub id: i64,
    pub start_date: String,
    pub start_date_local: String,
    pub timezone: String,
    pub utc_offset: f64,
    pub location_city: Option<serde_json::Value>,
    pub location_state: Option<serde_json::Value>,
    pub location_country: String,
    pub achievement_count: i64,
    pub kudos_count: i64,
    pub comment_count: i64,
    pub athlete_count: i64,
    pub photo_count: i64,
    pub map: Map,
    pub trainer: bool,
    pub commute: bool,
    pub manual: bool,
    pub private: bool,
    pub visibility: String,
    pub flagged: bool,
    pub gear_id: Option<serde_json::Value>,
    pub start_latlng: Vec<f64>,
    pub end_latlng: Vec<f64>,
    pub average_speed: f64,
    pub max_speed: f64,
    pub average_cadence: Option<f64>,
    pub average_temp: Option<i64>,
    pub average_watts: Option<f64>,
    pub max_watts: Option<i64>,
    pub weighted_average_watts: Option<i64>,
    pub kilojoules: Option<f64>,
    pub device_watts: Option<bool>,
    pub has_heartrate: bool,
    pub average_heartrate: Option<f64>,
    pub max_heartrate: Option<f64>,
    pub heartrate_opt_out: bool,
    pub display_hide_heartrate_option: bool,
    pub elev_high: Option<f64>,
    pub elev_low: Option<f64>,
    pub upload_id: Option<i64>,
    pub upload_id_str: Option<String>,
    pub external_id: Option<String>,
    pub from_accepted_tag: bool,
    pub pr_count: i64,
    pub total_photo_count: i64,
    pub has_kudoed: bool,
}

impl ActivityElement {
    /// Convert distance from km to miles
    pub fn distance_in_miles(&self, distance: f64) -> f64 {
        distance * 0.000621371
    }

    /// Convert moving time from seconds to mins
    pub fn moving_time_in_mins(&self, moving_time: f64) -> f64 {
        moving_time / 60.0
    }

    /// Convert elapsed time  from seconds to mins
    pub fn elapsed_time_in_mins(&self, elapsed_time: f64) -> f64 {
        elapsed_time / 60.0
    }
}

/// Athlete resource
#[derive(Serialize, Deserialize, Debug)]
pub struct Athlete {
    pub id: i64,
    pub resource_state: i32,
}

/// Map Resource
#[derive(Serialize, Deserialize, Debug)]
pub struct Map {
    pub id: String,
    pub summary_polyline: String,
    pub resource_state: i64,
}

/// Types of activity and sport returned from the API
#[derive(Serialize, Deserialize, Debug)]
pub enum ActivityType {
    Rowing,
    Run,
    Walk,
    WeightTraining,
    Ride,
    Swim,
    Hike,
    AlpineSki,
    BackcountrySki,
    Canoeing,
    Crossfit,
    EBikeRide,
    Elliptical,
    IceSkate,
    InlineSkate,
    Kayaking,
    Kitesurf,
    NordicSki,
    RockClimbing,
    RollerSki,
    Snowboard,
    Snowshoe,
    StairStepper,
    StandUpPaddling,
    Surfing,
    Windsurf,
    Workout,
    Yoga,
    Unknown,
}
