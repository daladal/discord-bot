use reqwest::{Client, StatusCode};
use std::time::Duration;
use super::types::{RiotAccount, Region};

pub struct RiotClient {
    api_key: String,
    http_client: Client,
}

#[derive(Debug)]
pub enum RiotApiError {
    NotFound,               // 404
    RateLimited,            // 429
    Unauthorized,           // 403
    ServerError,            // 500+
    NetworkError(String),   
    ParseError(String),
}

impl std::fmt::Display for RiotApiError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RiotApiError::NotFound => write!(f, "Summoner not found"),
            RiotApiError::RateLimited => write!(f, "Rate limited by Riot API"),
            RiotApiError::Unauthorized => write!(f, "Invalid API key"),
            RiotApiError::ServerError => write!(f, "Riot API server error"),
            RiotApiError::NetworkError(msg) => write!(f, "Network error: {}", msg),
            RiotApiError::ParseError(msg) => write!(f, "Parse error: {}", msg),
        }
    }
}

impl std::error::Error for RiotApiError {}

impl RiotClient {
    pub fn new(api_key: String) -> Self {
        let http_client = Client::builder()
            .timeout(Duration::from_secs(10))
            .build()
            .expect("Failed to create HTTP client");

        RiotClient {
            api_key,
            http_client
        }
    }

    pub async fn get_account_by_riot_id(&self, game_name: &str, tag_line: &str, game_region: &str) -> Result<RiotAccount, RiotApiError> {
        let region = Region::from_game_region(game_region);
        let base_url = region.api_base_url();

        let url = format!(
            "{}/riot/account/v1/accounts/by-riot-id/{}/{}",
            base_url, game_name, tag_line
        );

        let response = self.http_client
            .get(&url)
            .header("X-Riot-Token", &self.api_key)
            .send()
            .await
            .map_err(|e| RiotApiError::NetworkError(e.to_string()))?;
            
        match response.status() {
            StatusCode::OK => {
                let account = response
                    .json::<RiotAccount>()
                    .await
                    .map_err(|e| RiotApiError::ParseError(e.to_string()))?;
                Ok(account)
            }
            StatusCode::NOT_FOUND => Err(RiotApiError::NotFound),
            StatusCode::FORBIDDEN => Err(RiotApiError::Unauthorized),
            StatusCode::TOO_MANY_REQUESTS => Err(RiotApiError::RateLimited),
            status if status.is_server_error() => Err(RiotApiError::ServerError),
            status => Err(RiotApiError::NetworkError(format!("Unexpected status: {}", status))),
        }
    }
}















