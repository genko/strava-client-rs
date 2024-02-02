# strava-client-rs
Srava API Client in Rust that uses OAuth2 to get and refresh the access token

## About
This Rust lib will use the Stava API to both read and write against Strava

## Example
```rust
use strava_client_rs::api::oauth::auth;
use strava_client_rs::util::auth_config::config_file;
use std::env;
use std::path::Path;

fn main() {

    // Get the access token from the config file or get a new one
    let config_file = env::var("STRAVA_CONFIG_FILE").unwrap_or_else(|_| "config.json".to_string());
    let access_token = get_access_token(config_file).unwrap();

    // Print the access token
    println!("Access Token: {}", access_token);

  

}

// Get the access token from the config file or get a new one
fn get_access_token(config_file: String) -> Result<String, String>{
    let client_id = env::var("STRAVA_CLIENT_ID").expect("Missing the STRAVA_CLIENT_ID environment variable.");
    let client_secret = env::var("STRAVA_CLIENT_SECRET").expect("Missing the STRAVA_CLIENT_SECRET environment variable.");
    let auth_url = "http://www.strava.com/oauth/authorize";
    let token_url = "https://www.strava.com/oauth/token";

    // Setup default config for auth
    let mut config = auth::Config::new(
        client_id.to_string(),
        client_secret.to_string(),
        Default::default(), // no refresh token so set to default which is none
        auth_url.to_string(),
        token_url.to_string(),
    );

    // Check if the config file exists and get the access token or get a new one
    if Path::new(&config_file).exists() {
        config.refresh_token = Some(config_file::load_config().refresh_token);
        let refresh_access_token = auth::get_refresh_token(config);   
        Ok(refresh_access_token.unwrap().to_string())
    } else {
        let access_token = auth::get_authorization(config);     
        Ok(access_token.unwrap().to_string())
    }
```

## Disclamier
This library is not affiliated with Strava. Use at your own risk. 
This is very much a work in progress. Right now all it does is auth to the API and gets an access token.