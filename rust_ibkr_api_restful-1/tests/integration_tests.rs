// This file contains integration tests for the application, ensuring that the API client and scanning functionalities work as expected.

#[cfg(test)]
mod tests {
    use super::*;
    use crate::api::client::ApiClient;
    use crate::scanner::analyzer::analyze_option_chains;

    #[test]
    fn test_api_client_fetches_option_chains() {
        let client = ApiClient::new();
        let result = client.fetch_option_chains("AAPL"); // Example stock symbol
        assert!(result.is_ok());
        assert!(!result.unwrap().is_empty());
    }

    #[test]
    fn test_analyze_option_chains_for_profitability() {
        let option_chains = vec![
            // Mock data for option chains
            // Replace with actual data structure as defined in your models
        ];
        let profitable_strikes = analyze_option_chains(option_chains, 0.02); // 2% profit margin
        assert!(!profitable_strikes.is_empty());
    }
}