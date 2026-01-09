use reqwest::Client;
use anyhow::Result;

pub struct IbkrClient {
    client: Client,
    base_url: String,
}

impl IbkrClient {
    pub fn new(base_url: String) -> Self {
        Self {
            client: Client::new(),
            base_url,
        }
    }
    
    pub async fn get_option_chain(&self, symbol: &str) -> Result<String> {
        Ok(format!("Option chain for {}", symbol))
    }
}