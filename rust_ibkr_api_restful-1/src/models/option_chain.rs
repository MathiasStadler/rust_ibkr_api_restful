// This file defines the data structures representing an option chain, including fields for various options and methods for processing them.

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct OptionChain {
    pub symbol: String,
    pub options: Vec<Option>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Option {
    pub strike: f64,
    pub premium: f64,
    pub expiration: String,
    pub call: bool,
}

impl OptionChain {
    pub fn profitable_strikes(&self, profit_margin: f64) -> Vec<&Option> {
        self.options.iter()
            .filter(|option| {
                let profit = (option.strike - option.premium) / option.premium;
                profit >= profit_margin
            })
            .collect()
    }
}