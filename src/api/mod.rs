//! # Strava API module
//! Mods used to call the different Strava end-points
//! Documentation: https://developers.strava.com/docs/reference/

pub use self::helpers::strava_v3;
pub use self::oauth::auth;
pub use self::oauth::auth::get_authorization;
pub use self::oauth::auth::get_refresh_token;

pub mod oauth;

pub mod activities;
pub mod athlete;
pub mod club;
pub mod gear;
pub mod helpers;
