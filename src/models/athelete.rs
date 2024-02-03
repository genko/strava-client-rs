use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct AtheleteCollection {
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
    pub follower_count: i64,
    pub friend_count: i64,
    pub mutual_friend_count: i64,
    pub athlete_type: i64,
    pub date_preference: String,
    pub measurement_preference: String,
    pub clubs: Vec<Option<serde_json::Value>>,
    pub ftp: Option<serde_json::Value>,
    pub weight: i64,
    pub bikes: Vec<Bike>,
    pub shoes: Vec<Bike>,
}

#[derive(Serialize, Deserialize)]
pub struct Bike {
    pub id: String,
    pub primary: bool,
    pub name: String,
    pub resource_state: i64,
    pub distance: i64,
}
