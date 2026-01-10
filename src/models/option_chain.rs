use chrono::{Utc, NaiveDate, FixedOffset};
use chrono_tz::US::Eastern;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OptionChain {
    pub symbol: String,
    pub current_price: f64,
    pub earnings_date: Option<NaiveDate>,  // Earnings-Datum
    pub strikes: Vec<Strike>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Strike {
    pub price: f64,
    pub bid: f64,
    pub ask: f64,
    pub premium: f64,
    pub delta: f64,
    pub expiration_date: NaiveDate,
}

impl Strike {
    pub fn days_to_expiration(&self) -> i64 {
        let today = Utc::now().with_timezone(&Eastern).naive_local().date();
        (self.expiration_date - today).num_days()
    }
    
    pub fn is_otm_with_low_delta(&self, current_price: f64, max_delta: f64) -> bool {
        let is_otm = self.price > current_price;
        let low_delta = self.delta <= max_delta && self.delta > 0.0;
        is_otm && low_delta
    }
    
    /// Prüfe ob Earnings-Datum während Option-Laufzeit liegt
    pub fn earnings_during_option_life(&self, earnings_date: Option<NaiveDate>) -> bool {
        match earnings_date {
            Some(earnings) => {
                let today = Utc::now().naive_utc().date();
                // Earnings liegt zwischen heute und Expiration
                earnings > today && earnings <= self.expiration_date
            }
            None => false,
        }
    }
}