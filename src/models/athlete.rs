/// # Athlete model
/// This model is used to represent the athlete object returned by the Strava API
/// Documentation: https://developers.strava.com/docs/reference/#api-models-DetailedAthlete
/// Contains helper methods to convert units and get athlete information
use serde::{Deserialize, Serialize};

/// Athlete fields returned from the API
#[derive(Serialize, Deserialize, Debug)]
pub struct AthleteCollection {
    pub id: f64,
    pub username: String,
    pub resource_state: i64,
    pub firstname: String,
    pub lastname: String,
    pub city: String,
    pub state: String,
    pub country: String,
    pub sex: String,
    pub premium: bool,
    pub created_at: String,
    pub updated_at: String,
    pub badge_type_id: i64,
    pub profile_medium: String,
    pub profile: String,
    pub friend: Option<serde_json::Value>,
    pub follower: Option<serde_json::Value>,
    pub follower_count: Option<i64>,
    pub friend_count: Option<i64>,
    pub mutual_friend_count: Option<i64>,
    pub athlete_type: Option<i64>,
    pub date_preference: Option<String>,
    pub measurement_preference: Option<String>,
    pub clubs: Option<Vec<Option<serde_json::Value>>>,
    pub ftp: Option<serde_json::Value>,
    pub weight: f64,
    // weight in kg
    pub bikes: Option<Vec<Gear>>,
    pub shoes: Option<Vec<Gear>>,
}

impl AthleteCollection {
    /// Returns the full name of athlete using the firstname and lastname fields
    pub fn get_full_name(&self) -> String {
        format!("{} {}", self.firstname, self.lastname)
    }

    /// Returns the id of the athlete
    pub fn get_id(&self) -> f64 {
        self.id
    }
    /// Returns the username of the athlete
    pub fn get_username(&self) -> String {
        self.username.clone()
    }
    /// Returns the weight in kg of the athlete
    pub fn get_weight(&self) -> f64 {
        self.weight
    }
    /// Converts the weight of the athlete into lb from kg
    pub fn weight_in_lbs(&self) -> f64 {
        self.weight * 2.20462
    }
    /// Returns bool of premium status of athlete
    pub fn get_premium(&self) -> bool {
        self.premium
    }
    /// Returns the sex of the athlete
    pub fn get_sex(&self) -> String {
        self.sex.clone()
    }
    /// Returns the city of the athlete
    pub fn get_city(&self) -> String {
        self.city.clone()
    }
    /// Returns the state of the athlete
    pub fn get_state(&self) -> String {
        self.state.clone()
    }
    /// Returns the distance in miles converting from km of the athlete
    pub fn distance_in_miles(&self, distance: f64) -> f64 {
        distance * 0.000621371
    }
    /// Returns the shoe Vec of the athlete
    pub fn get_shoes(&self) -> &Vec<Gear> {
        self.shoes.as_ref().unwrap()
    }
    /// Returns the bike vec of the athlete
    pub fn get_bikes(&self) -> &Vec<Gear> {
        self.bikes.as_ref().unwrap()
    }
}

/// Used for both shoes and bikes
#[derive(Serialize, Deserialize, Debug)]
pub struct Gear {
    pub id: String,
    pub primary: bool,
    pub name: String,
    pub resource_state: i64,
    pub distance: f64, // distance in meters
}

/// Athlete stats data
#[derive(Serialize, Deserialize, Debug)]
pub struct AthleteStats {
    pub biggest_ride_distance: f64,
    pub biggest_climb_elevation_gain: Option<serde_json::Value>,
    pub recent_ride_totals: Totals,
    pub all_ride_totals: Totals,
    pub recent_run_totals: Totals,
    pub all_run_totals: Totals,
    pub recent_swim_totals: Totals,
    pub all_swim_totals: Totals,
    pub ytd_ride_totals: Totals,
    pub ytd_run_totals: Totals,
    pub ytd_swim_totals: Totals,
}

/// Athlete totals for each stat
#[derive(Serialize, Deserialize, Debug)]
pub struct Totals {
    pub count: i64,
    pub distance: f64,
    // distance in meters
    pub moving_time: i64,
    // time in seconds
    pub elapsed_time: i64,
    pub elevation_gain: f64,
    pub achievement_count: Option<i64>,
}
