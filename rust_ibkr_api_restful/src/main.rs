// src/main.rs

use std::error::Error;
use rust_ibkr_api_restful::api::client::ApiClient;
use rust_ibkr_api_restful::scanner::analyzer::analyze_option_chains;

fn main() -> Result<(), Box<dyn Error>> {
    // Initialize the API client
    let api_client = ApiClient::new()?;
    
    // Fetch the NASDAQ option chains
    let option_chains = api_client.fetch_nasdaq_option_chains()?;
    
    // Analyze the option chains for profitable strikes
    let profitable_strikes = analyze_option_chains(option_chains, 0.02)?;
    
    // Print the profitable strikes
    for strike in profitable_strikes {
        println!("{:?}", strike);
    }
    
    Ok(())
}