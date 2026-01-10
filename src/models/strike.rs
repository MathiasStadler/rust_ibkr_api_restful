use chrono::{NaiveDate, Utc};
use chrono_tz::US::Eastern;

#[derive(Debug, Clone)]
pub struct ProfitableStrike {
    pub symbol: String,
    pub strike_price: f64,
    pub premium: f64,
    pub profit_percentage: f64,
    pub delta: f64,
    pub expiration_date: NaiveDate,
    pub days_to_expiration: i64,
    pub earnings_date: Option<NaiveDate>,           // Earnings-Datum
    pub has_earnings_during_option: bool,           // Marker für Earnings während Laufzeit
    pub days_to_earnings: Option<i64>,              // Tage bis Earnings
}

impl ProfitableStrike {
    pub fn new(
        symbol: &str,
        strike_price: f64,
        premium: f64,
        delta: f64,
        expiration_date: NaiveDate,
        earnings_date: Option<NaiveDate>,
    ) -> Self {
        let profit_percentage = (premium / strike_price) * 100.0;
        
        // Richtig: .date() gibt dir das Datum IN Eastern Time zurück
        let today = Utc::now()
            .with_timezone(&Eastern)
            .date();  // ← .date() nicht .date_naive()!
        let today = today.naive_utc();
        
        let days_to_expiration = (expiration_date - today).num_days();
        
        // Prüfe ob Earnings während Option-Laufzeit
        let has_earnings_during_option = match earnings_date {
            Some(earnings) => earnings > today && earnings <= expiration_date,
            None => false,
        };
        
        // Berechne Tage bis Earnings
        let days_to_earnings = earnings_date.map(|earnings| (earnings - today).num_days());
        
        Self {
            symbol: symbol.to_string(),
            strike_price,
            premium,
            profit_percentage,
            delta,
            expiration_date,
            days_to_expiration,
            earnings_date,
            has_earnings_during_option,
            days_to_earnings,
        }
    }
}

impl std::cmp::PartialOrd for ProfitableStrike {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        other.premium.partial_cmp(&self.premium)
    }
}

impl std::cmp::PartialEq for ProfitableStrike {
    fn eq(&self, other: &Self) -> bool {
        (self.premium - other.premium).abs() < f64::EPSILON
    }
}