// src/api/client.rs

use reqwest::Client;
use serde::Deserialize;
use std::error::Error;

const IBKR_API_URL: &str = "https://api.ibkr.com/v1.0";

pub struct ApiClient {
    client: Client,
    api_key: String,
}

impl ApiClient {
    pub fn new(api_key: String) -> Self {
        let client = Client::new();
        ApiClient { client, api_key }
    }

    pub async fn fetch_option_chain(&self, symbol: &str) -> Result<OptionChainResponse, Box<dyn Error>> {
        let url = format!("{}/option_chain/{}", IBKR_API_URL, symbol);
        let response = self.client
            .get(&url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .send()
            .await?
            .json::<OptionChainResponse>()
            .await?;
        
        Ok(response)
    }
}

#[derive(Deserialize)]
pub struct OptionChainResponse {
    // Define fields based on the API response structure
    // Example:
    // options: Vec<OptionData>,
}

#[derive(Deserialize)]
pub struct OptionData {
    // Define fields for individual option data
    // Example:
    // strike: f64,
    // premium: f64,
}