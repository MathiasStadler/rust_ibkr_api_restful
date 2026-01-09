// This file contains the logic for analyzing option chains, including functions to identify profitable strikes based on a specified profit margin.

use crate::models::option_chain::OptionChain;
use crate::models::strike::Strike;

pub struct Analyzer {
    profit_margin: f64,
}

impl Analyzer {
    pub fn new(profit_margin: f64) -> Self {
        Analyzer { profit_margin }
    }

    pub fn analyze(&self, option_chain: &OptionChain) -> Vec<Strike> {
        option_chain.strikes.iter()
            .filter(|strike| self.is_profitable(strike))
            .cloned()
            .collect()
    }

    fn is_profitable(&self, strike: &Strike) -> bool {
        let profit_threshold = strike.premium * (1.0 + self.profit_margin);
        strike.strike_value >= profit_threshold
    }
}