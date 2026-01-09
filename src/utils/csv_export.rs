use crate::models::strike::ProfitableStrike;
use std::fs::File;
use anyhow::Result;
use chrono::Local;

pub struct CsvExporter;

impl CsvExporter {
    /// Exportiere profitable Strikes zu CSV-Datei
    pub fn export_to_csv(strikes: &[ProfitableStrike], filename: Option<&str>) -> Result<String> {
        let default_filename = format!(
            "option_scan_results_{}.csv",
            Local::now().format("%Y%m%d_%H%M%S")
        );
        
        let filepath = filename.unwrap_or(&default_filename);
        let file = File::create(filepath)?;
        let mut writer = csv::Writer::from_writer(file);
        
        // Schreibe Header
        writer.write_record(&[
            "Index",
            "Symbol",
            "Strike Price",
            "Premium",
            "Profit %",
            "Delta",
            "Expiration Date",
            "Days to Expiration",
            "Earnings Date",
            "Earnings During Option",
            "Days to Earnings",
        ])?;
        
        // Schreibe Daten
        for (idx, strike) in strikes.iter().enumerate() {
            let earnings_during = if strike.has_earnings_during_option {
                "YES"
            } else {
                "NO"
            };
            
            let days_to_earnings = strike.days_to_earnings
                .map(|d| d.to_string())
                .unwrap_or_else(|| "N/A".to_string());
            
            let earnings_date = strike.earnings_date
                .map(|d| d.to_string())
                .unwrap_or_else(|| "N/A".to_string());
            
            writer.write_record(&[
                (idx + 1).to_string(),
                strike.symbol.clone(),
                format!("{:.2}", strike.strike_price),
                format!("{:.2}", strike.premium),
                format!("{:.2}", strike.profit_percentage),
                format!("{:.2}", strike.delta),
                strike.expiration_date.to_string(),
                strike.days_to_expiration.to_string(),
                earnings_date,
                earnings_during.to_string(),
                days_to_earnings,
            ])?;
        }
        
        writer.flush()?;
        Ok(filepath.to_string())
    }
}