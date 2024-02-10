// Helper functions for the API
use log::warn;
use reqwest::StatusCode;

// Function to handle API errors
pub fn handle_api_error(response: StatusCode) -> Result<(), Box<dyn std::error::Error>> {
    if response.is_client_error() {
        warn!("API response: {:?}", response);
        return Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::Other,
            "API returned an error",
        )));
    }
    Ok(())
}

// Returns the URL for the Strava API
pub fn strava_v3(path: String) -> String {
    format!("https://www.strava.com/api/v3/{}", path)
}
