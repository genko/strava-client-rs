//!  # Strava_client_rs Models
//! Models used to serialize and deserialize the json objects returned from the API

pub use self::activities::ActivityCollection;
pub use self::athlete::AthleteCollection;
pub use self::gear::GearCollection;

pub mod athlete;

pub mod activities;
pub mod clubs;
pub mod gear;
