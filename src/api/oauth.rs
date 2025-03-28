//! # OAuth
//! Module: oauth
//! Contains the Config struct and the get_authorization and get_refresh_token functions.
//! Calls the redirect server to get the code and state from the redirect URL.
//! Calls the config_file module to write the access token and refresh token to the config file.
//! Calls the reqwest module to make the request to the Strava API to get the access token and refresh token.

use crate::util::{auth_config, redirect};
use log::{info, trace, warn};
use oauth2::basic::BasicClient;
use oauth2::reqwest::http_client;
use oauth2::{
    AuthUrl, AuthorizationCode, ClientId, ClientSecret, CsrfToken, RedirectUrl, RefreshToken,
    RequestTokenError, Scope, TokenResponse, TokenUrl,
};

/// Module: auth
/// Contains the Config struct and the get_authorization and get_refresh_token functions.
/// Calls the redirect server to get the code and state from the redirect URL.
/// Calls the config_file module to write the access token and refresh token to the config file.
/// Calls the reqwest module to make the request to the Strava API to get the access token and refresh token.
pub mod auth {
    use super::*;

    /// Config struct to hold the client_id, client_secret, refresh_token, auth_url, and token_url
    /// Passed to the get_authorization and get_refresh_token functions.
    #[derive(Clone, Default, Debug, PartialEq)]
    pub struct Config {
        pub client_id: String,
        pub client_secret: String,
        pub refresh_token: Option<String>,
        pub auth_url: String,
        pub token_url: String,
    }

    /// Implementation of the Config struct to create a new Config struct.
    /// Refresh token is set to Some(refresh_token) to allow for the refresh token to be set to None.
    impl Config {
        pub fn new(
            client_id: String,
            client_secret: String,
            refresh_token: String,
            auth_url: String,
            token_url: String,
        ) -> Config {
            Config {
                client_id,
                client_secret,
                refresh_token: Some(refresh_token),
                auth_url,
                token_url,
            }
        }
    }

    /// Get the authorization code from the redirect URL and exchange it for an access token and refresh token.
    /// The access token and refresh token are written to the config file.
    /// The access token is returned.
    /// The refresh token is used to get a new access token and refresh token.
    /// Arguments: Config struct
    /// Returns: Result<String, String>
    pub fn get_authorization(config: Config) -> Result<String, String> {
        // Set the client_id, client_secret, auth_url, and token_url
        let strava_client_id = ClientId::new(config.client_id.clone());
        let strava_client_secret = ClientSecret::new(config.client_secret.clone());
        let auth_url = AuthUrl::new(config.auth_url).expect("Invalid authorization endpoint URL");
        let token_url = TokenUrl::new(config.token_url).expect("Invalid token endpoint URL");

        // Set the redirect URL this is where the code and state will be sent to
        // The redirect module is configured to listen on localhost:8888 for the redirect URL
        let redirect_url =
            RedirectUrl::new("http://localhost:8888".to_string()).expect("Invalid redirect URL");

        // Create a new BasicClient with the client_id, client_secret, auth_url, and token_url
        let client = BasicClient::new(
            strava_client_id,
            Some(strava_client_secret),
            auth_url,
            Some(token_url),
        )
        .set_redirect_uri(redirect_url);

        info!(
            "Authorization URL: {:?}",
            client.authorize_url(CsrfToken::new_random).url()
        );

        // Create the authorization URL with the CSRF token
        let (authorize_url, csrf_state) = client
            .authorize_url(CsrfToken::new_random)
            .add_extra_param("exchange_token&approval_prompt", "force")
            .add_scope(Scope::new(
                "profile:read_all,activity:read_all,profile:write".to_string(),
            ))
            .url();

        // Print the authorization URL and open it in the browser on the terminal
        println!("Open this URL in your browser:\n{}\n", authorize_url);

        // Get the code and state from the redirect URL
        let get_response_params = redirect::server::run();

        trace!("Code: {}", get_response_params.get("code").unwrap());
        trace!("State: {}", get_response_params.get("state").unwrap());

        // Create the AuthorizationCode and CsrfToken from the code and state
        let code = AuthorizationCode::new(get_response_params.get("code").unwrap().to_string());
        let state = CsrfToken::new(get_response_params.get("state").unwrap().to_string());

        if state.secret() != csrf_state.secret() {
            warn!("CSRF token mismatch");
            panic!("CSRF token mismatch");
        }

        // Exchange the code for an access token and refresh token
        let token_res = client
            .exchange_code(code)
            .add_extra_param("client_id", config.client_id)
            .add_extra_param("client_secret", config.client_secret)
            .request(http_client)
            .map_err(|e| {
                match e {
                    RequestTokenError::ServerResponse(provider_err) => {
                        warn!("Server returned error response: {:?}", provider_err)
                    }
                    RequestTokenError::Request(req) => {
                        warn!("Request failed: {:?}", req)
                    }
                    RequestTokenError::Parse(parse_err, res) => {
                        let body = match std::str::from_utf8(&res) {
                            Ok(text) => text.to_string(),
                            Err(_) => format!("{:?}", &res),
                        };
                        warn!(
                            "Failed to parse server response: {} [response={:?}]",
                            parse_err, body
                        )
                    }
                    RequestTokenError::Other(_msg) => {
                        warn!("Failed to perform request: {}", _msg)
                    }
                };
            });

        let token_res = token_res.unwrap();
        let refresh_token = token_res.refresh_token().unwrap().secret().to_string();
        let access_token = token_res.access_token().secret().to_string();

        trace!("Access Token: {}", access_token);
        trace!("Refresh Token: {}", refresh_token);

        // Write the access token and refresh token to the config file
        auth_config::config_file::write_config(&access_token, &refresh_token);

        Ok(access_token)
    }

    /// Public function to get the refresh token from the config file.
    /// The refresh token is used to get a new access token and refresh token.
    /// Arguments: Config struct
    /// Returns: Result<String, String>
    pub fn get_refresh_token(config: Config) -> Result<String, String> {
        // Set the client_id, client_secret, auth_url, and token_url, and refresh_token from the config struct
        let strava_client_id = ClientId::new(config.client_id.clone());
        let strava_client_secret = ClientSecret::new(config.client_secret.clone());
        let auth_url = AuthUrl::new(config.auth_url).expect("Invalid authorization endpoint URL");
        let token_url = TokenUrl::new(config.token_url).expect("Invalid token endpoint URL");
        let refresh_token = RefreshToken::new(config.refresh_token.expect("Missing refresh token"));

        // Create a new BasicClient with the client_id, client_secret, auth_url, and token_url
        let client = BasicClient::new(
            strava_client_id,
            Some(strava_client_secret),
            auth_url,
            Some(token_url),
        );

        // Exchange the refresh token for a new access token and refresh token
        let refresh_res = client
            .exchange_refresh_token(&refresh_token)
            .add_extra_param("client_id", config.client_id)
            .add_extra_param("client_secret", config.client_secret)
            .request(http_client);

        let refresh_res = refresh_res.unwrap();
        let refresh_token = refresh_res.refresh_token().unwrap().secret().to_string();
        let access_token = refresh_res.access_token().secret().to_string();

        trace!("Access Token: {}", access_token);
        trace!("Refresh Token: {}", refresh_token);

        // Write the access token and refresh token to the config file
        auth_config::config_file::write_config(&access_token, &refresh_token);
        Ok(access_token)
    }
}
