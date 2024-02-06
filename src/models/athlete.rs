use serde::{Serialize, Deserialize};

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
    pub bikes: Option<Vec<Bike>>,
    pub shoes: Option<Vec<Bike>>,
}

impl AthleteCollection {
    pub fn get_full_name(&self) -> String {
        format!("{} {}", self.firstname, self.lastname)
    }

    pub fn get_id(&self) -> f64 {
        self.id
    }

    pub fn get_username(&self) -> String {
        self.username.clone()
    }

    pub fn get_weight(&self) -> f64 {
        self.weight
    }

    pub fn get_premium(&self) -> bool {
        self.premium
    }

    pub fn get_sex(&self) -> String {
        self.sex.clone()
    }

    pub fn get_city(&self) -> String {
        self.city.clone()
    }

    pub fn get_state(&self) -> String {
        self.state.clone()
    }

}

#[derive(Serialize, Deserialize, Debug)]
pub struct Bike {
    pub id: String,
    pub primary: bool,
    pub name: String,
    pub resource_state: i64,
    pub distance: f64,
}

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

#[derive(Serialize, Deserialize, Debug)]
pub struct Totals {
    pub count: i64,
    pub distance: f64,
    pub moving_time: i64,
    pub elapsed_time: i64,
    pub elevation_gain: f64,
    pub achievement_count: Option<i64>,
}
