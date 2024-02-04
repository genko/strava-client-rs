use serde::{Serialize, Deserialize};

pub type ActvityCollection = Vec<ActivityElement>;

#[derive(Serialize, Deserialize)]
pub struct ActivityElement {
    pub resource_state: i64,
    pub athlete: Athlete,
    pub name: String,
    pub distance: f64,
    pub moving_time: i64,
    pub elapsed_time: i64,
    pub total_elevation_gain: i64,
    #[serde(rename = "type")]
    pub welcome_type: Type,
    pub sport_type: Type,
    pub workout_type: Option<serde_json::Value>,
    pub id: i64,
    pub start_date: String,
    pub start_date_local: String,
    pub timezone: Timezone,
    pub utc_offset: i64,
    pub location_city: Option<serde_json::Value>,
    pub location_state: Option<serde_json::Value>,
    pub location_country: LocationCountry,
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
    pub visibility: Visibility,
    pub flagged: bool,
    pub gear_id: Option<GearId>,
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
    pub max_heartrate: Option<i64>,
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

#[derive(Serialize, Deserialize)]
pub struct Athlete {
    pub id: i64,
    pub resource_state: i64,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum GearId {
    G9437312,
}

#[derive(Serialize, Deserialize)]
pub enum LocationCountry {
    #[serde(rename = "United States")]
    UnitedStates,
}

#[derive(Serialize, Deserialize)]
pub struct Map {
    pub id: String,
    pub summary_polyline: String,
    pub resource_state: i64,
}

#[derive(Serialize, Deserialize)]
pub enum Type {
    Rowing,
    Run,
    Walk,
    #[serde(rename = "WeightTraining")]
    WeightTraining,
}

#[derive(Serialize, Deserialize)]
pub enum Timezone {
    #[serde(rename = "(GMT-05:00) America/Atikokan")]
    Gmt0500AmericaAtikokan,
    #[serde(rename = "(GMT-05:00) America/New_York")]
    Gmt0500AmericaNewYork,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Visibility {
    Everyone,
}