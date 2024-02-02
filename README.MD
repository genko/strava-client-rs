# strava-client-rs
Srava API Client in Rust that uses OAuth2 to get and refresh the access token

## About
This Rust lib will use the Stava API to both read and write against Strava

## Example
```rust
use strava_client_rs::oauth::auth;
use std::env;

fn main() {
    let client_id = env::var("STRAVA_CLIENT_ID").expect("Missing the STRAVA_CLIENT_ID environment variable.");
    let client_secret = env::var("STRAVA_CLIENT_SECRET").expect("Missing the STRAVA_CLIENT_SECRET environment variable.");
    let auth_url = "http://www.strava.com/oauth/authorize";
    let token_url = "https://www.strava.com/oauth/token";

    let config = auth::Config::new(
        client_id.to_string(),
        client_secret.to_string(),
        auth_url.to_string(),
        token_url.to_string(),
    );
    let token = auth::get_authorization(config);
    println!("Token: {}", token);


}
```

## Disclamier
This library is not affiliated with Strava. Use at your own risk. 
This is very much a work in progress. Right now all it does is auth to the API and gets an access token.