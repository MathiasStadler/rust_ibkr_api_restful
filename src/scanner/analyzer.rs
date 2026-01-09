use crate::models::option_chain::OptionChain;
use crate::models::strike::ProfitableStrike;

pub struct OptionAnalyzer;

impl OptionAnalyzer {
    /// Findet die besten Optionen mit:
    /// - Höchster Prämie
    /// - Delta ≤ 0.30 (Out-of-the-Money)
    /// - Spezifische Laufzeit
    /// - Markiert Optionen mit Earnings während Laufzeit
    pub fn find_best_otm_strikes(
        chain: &OptionChain,
        min_profit: f64,
        max_delta: f64,
        min_days: i64,
        max_days: i64,
        top_n: usize,
    ) -> Vec<ProfitableStrike> {
        let mut candidates: Vec<ProfitableStrike> = chain
            .strikes
            .iter()
            .filter_map(|strike| {
                let days_to_exp = strike.days_to_expiration();

                // Filter 1: Laufzeit (z.B. 7-45 Tage)
                if days_to_exp < min_days || days_to_exp > max_days {
                    return None;
                }

                // Filter 2: Out-of-the-Money mit Delta ≤ 0.30
                if !strike.is_otm_with_low_delta(chain.current_price, max_delta) {
                    return None;
                }

                let profitable = ProfitableStrike::new(
                    &chain.symbol,
                    strike.price,
                    strike.premium,
                    strike.delta,
                    strike.expiration_date,
                    chain.earnings_date, // Übergebe Earnings-Datum
                );

                // Filter 3: Mindestgewinn (2%)
                if profitable.profit_percentage >= min_profit {
                    Some(profitable)
                } else {
                    None
                }
            })
            .collect();

        // Sortiere nach Prämie (höchste zuerst)
        candidates.sort_by(|a, b| b.premium.partial_cmp(&a.premium).unwrap());

        // Gib nur Top N zurück
        candidates.into_iter().take(top_n).collect()
    }
}