// This file defines the gateway for the IBKR API, handling authentication and session management.

use reqwest::Client;
use std::error::Error;

pub struct Gateway {
    client: Client,
    api_key: String,
    api_secret: String,
    base_url: String,
}

impl Gateway {
    pub fn new(api_key: String, api_secret: String) -> Result<Self, Box<dyn Error>> {
        let client = Client::new();
        let base_url = "https://api.ibkr.com/v1".to_string();

        Ok(Gateway {
            client,
            api_key,
            api_secret,
            base_url,
        })
    }

    pub async fn authenticate(&self) -> Result<(), Box<dyn Error>> {
        // Implement authentication logic here
        Ok(())
    }

    pub async fn fetch_option_chain(&self, symbol: &str) -> Result<String, Box<dyn Error>> {
        let url = format!("{}/option_chain/{}", self.base_url, symbol);
        let response = self.client.get(&url)
            .header("API-Key", &self.api_key)
            .header("API-Secret", &self.api_secret)
            .send()
            .await?;

        let body = response.text().await?;
        Ok(body)
    }
}