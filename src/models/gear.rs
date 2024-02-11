//! Gear model
//! This model is used to represent the gear object returned by the Strava API
//! Documentation: https://developers.strava.com/docs/reference/#api-models-Gear

use serde::{Deserialize, Serialize};

/// Gear fields returned from the API
#[derive(Debug, Serialize, Deserialize)]
pub struct GearCollection {
    pub id: String,
    pub primary: bool,
    pub name: String,
    pub nickname: String,
    pub resource_state: i64,
    pub retired: bool,
    pub distance: i64,
    pub converted_distance: f64,
    pub brand_name: String,
    pub model_name: String,
    pub description: Option<serde_json::Value>,
    pub notification_distance: i64,
}
