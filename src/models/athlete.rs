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

#[derive(Serialize, Deserialize, Debug)]
pub struct Bike {
    pub id: String,
    pub primary: bool,
    pub name: String,
    pub resource_state: i64,
    pub distance: f64,
}
