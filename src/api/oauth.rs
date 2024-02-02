use crate::util::auth_config;
use crate::util::redirect;
use oauth2::basic::BasicClient;
use oauth2::reqwest::http_client;
use oauth2::{
    AuthUrl, AuthorizationCode, ClientId, ClientSecret, CsrfToken, RedirectUrl, Scope, RequestTokenError,
    TokenUrl, RefreshToken
};

pub mod auth {

    use oauth2::TokenResponse;

    use super::*;

    #[derive(Clone, Default, Debug, PartialEq)]
    pub struct Config {
        pub client_id: String,
        pub client_secret: String,
        pub refresh_token: Option<String>,
        pub auth_url: String,
        pub token_url: String,
    }

    impl Config {
        pub fn new(client_id: String, client_secret: String, refresh_token: String, auth_url: String, token_url: String) -> Config {
            Config {
                client_id,
                client_secret,
                refresh_token: Some(refresh_token),
                auth_url,
                token_url,
            }
        }
    }

    pub fn get_authorization(config: Config) -> Result<String, String> {
        
        let token_config = config.clone(); // This is a temporary fix to get the client_id and client_secret into the token request

        let strava_client_id = ClientId::new(config.client_id);
        let strava_client_secret = ClientSecret::new(config.client_secret);
        let auth_url = AuthUrl::new(config.auth_url).expect("Invalid authorization endpoint URL");
        let token_url = TokenUrl::new(config.token_url).expect("Invalid token endpoint URL");
        let redirect_url = RedirectUrl::new("http://localhost:8888".to_string()).expect("Invalid redirect URL");

        let client = BasicClient::new(
            strava_client_id,
            Some(strava_client_secret),
            auth_url,
            Some(token_url),
        ).
        set_redirect_uri(redirect_url);

        let (authorize_url, csrf_state) = client
            .authorize_url(CsrfToken::new_random)
            .add_extra_param("exchange_token&approval_prompt", "force")
            .add_scope(Scope::new("read".to_string()))
            .url();

            println!(
                "Open this URL in your browser:\n{}\n",
                authorize_url.to_string()
            );
        let get_response_params = redirect::server::run();
        let code = AuthorizationCode::new(get_response_params.get("code").unwrap().to_string());
        let state = CsrfToken::new(get_response_params.get("state").unwrap().to_string());

        if state.secret() != csrf_state.secret() {
            panic!("CSRF token mismatch");
        }

        let token_res = client.exchange_code(code).
        add_extra_param("client_id", token_config.client_id).
        add_extra_param("client_secret", token_config.client_secret).
        request(http_client).
        map_err(|e| {
            let _msg = match e {
                RequestTokenError::ServerResponse(provider_err) => {
                    println!("Server returned error response: {:?}", provider_err)
                },
                RequestTokenError::Request(req) => {
                    println!("Request failed: {:?}", req)
                },
                RequestTokenError::Parse(parse_err, res) => {
                    let body = match std::str::from_utf8(&res) {
                        Ok(text) => text.to_string(),
                        Err(_) => format!("{:?}", &res),
                    };
                    println!("Failed to parse server response: {} [response={:?}]",
                        parse_err, body)
                },
                RequestTokenError::Other(_msg) => {
                    println!("Failed to perform request: {}", _msg)
                },
            };
        });
        let refresh_token = token_res.as_ref().unwrap().refresh_token().unwrap().secret().to_string();
        let access_token = token_res.unwrap().access_token().secret().to_string();
        auth_config::config_file::write_config(&access_token, &refresh_token);
        Ok(access_token)
    }

    pub fn get_refresh_token(config: Config) -> Result<String, String> {

        let token_config = config.clone(); // This is a temporary fix to get the client_id and client_secret into the token request

        let strava_client_id = ClientId::new(config.client_id);
        let strava_client_secret = ClientSecret::new(config.client_secret);
        let auth_url = AuthUrl::new(config.auth_url).expect("Invalid authorization endpoint URL");
        let token_url = TokenUrl::new(config.token_url).expect("Invalid token endpoint URL");
        let refresh_token = RefreshToken::new(config.refresh_token.unwrap());


        let client = BasicClient::new(
            strava_client_id,
            Some(strava_client_secret),
            auth_url,
            Some(token_url),
        );

        let refresh_res = client.
        exchange_refresh_token(&refresh_token).
        add_extra_param("client_id", token_config.client_id).
        add_extra_param("client_secret", token_config.client_secret).
        request(http_client);

        let refresh_token = refresh_res.as_ref().unwrap().refresh_token().unwrap().secret().to_string();
        let access_token = refresh_res.as_ref().unwrap().access_token().secret().to_string();
        auth_config::config_file::write_config(&access_token, &refresh_token);
        Ok(access_token)

    }


}