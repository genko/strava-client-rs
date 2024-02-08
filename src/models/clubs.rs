use serde::{Serialize, Deserialize};

pub type Club = Vec<ClubCollection>;
#[derive(Debug, Serialize, Deserialize)]
pub struct ClubCollection {
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
