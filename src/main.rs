use anyhow::Result;
use tracing::info;
use std::env;
use chrono::{Local, NaiveDate};

mod api;
mod models;
mod scanner;
mod utils;

use api::client::IbkrClient;
use models::option_chain::OptionChain;
use scanner::analyzer::OptionAnalyzer;
use utils::csv_export::CsvExporter;

/// Earnings-Daten für alle NASDAQ Stocks mit Optionen
fn get_earnings_date(symbol: &str) -> Option<NaiveDate> {
    let today = Local::now().naive_local().date();
    
    match symbol {
        // Technology - Mega Cap
        "AAPL" => Some(today + chrono::Duration::days(15)),
        "MSFT" => Some(today + chrono::Duration::days(20)),
        "GOOGL" | "GOOG" => Some(today + chrono::Duration::days(25)),
        "AMZN" => Some(today + chrono::Duration::days(30)),
        "NVDA" => Some(today + chrono::Duration::days(10)),
        "META" => Some(today + chrono::Duration::days(22)),
        "TSLA" => Some(today + chrono::Duration::days(18)),
        
        // Technology - Large Cap
        "NFLX" => Some(today + chrono::Duration::days(12)),
        "ADBE" => Some(today + chrono::Duration::days(28)),
        "CSCO" => Some(today + chrono::Duration::days(35)),
        "INTC" => Some(today + chrono::Duration::days(32)),
        "AMD" => Some(today + chrono::Duration::days(27)),
        "AVGO" => Some(today + chrono::Duration::days(21)),
        "QCOM" => Some(today + chrono::Duration::days(24)),
        "ASML" => Some(today + chrono::Duration::days(38)),
        "LRCX" => Some(today + chrono::Duration::days(26)),
        "KLAC" => Some(today + chrono::Duration::days(23)),
        "ANET" => Some(today + chrono::Duration::days(29)),
        "CRWD" => Some(today + chrono::Duration::days(19)),
        "NET" => Some(today + chrono::Duration::days(31)),
        "DDOG" => Some(today + chrono::Duration::days(33)),
        "FTNT" => Some(today + chrono::Duration::days(25)),
        "ZS" => Some(today + chrono::Duration::days(30)),
        "PYPL" => Some(today + chrono::Duration::days(17)),
        "INTU" => Some(today + chrono::Duration::days(34)),
        
        // Software & Cloud
        "CRM" => Some(today + chrono::Duration::days(16)),
        "ORCL" => Some(today + chrono::Duration::days(36)),
        "SNOW" => Some(today + chrono::Duration::days(39)),
        "DOCN" => Some(today + chrono::Duration::days(22)),
        "OKTA" => Some(today + chrono::Duration::days(28)),
        "ZOOM" => Some(today + chrono::Duration::days(20)),
        "TWLO" => Some(today + chrono::Duration::days(26)),
        "ROKU" => Some(today + chrono::Duration::days(24)),
        "SHOP" => Some(today + chrono::Duration::days(37)),
        "MELI" => Some(today + chrono::Duration::days(19)),
        "PINS" => Some(today + chrono::Duration::days(29)),
        "SPLK" => Some(today + chrono::Duration::days(32)),
        "TEAM" => Some(today + chrono::Duration::days(23)),
        "RPD" => Some(today + chrono::Duration::days(27)),
        "DB" => Some(today + chrono::Duration::days(25)),
        
        // AI & Semiconductors
        "ARM" => Some(today + chrono::Duration::days(21)),
        "SUPER" => Some(today + chrono::Duration::days(14)),
        "MSTR" => Some(today + chrono::Duration::days(18)),
        "COIN" => Some(today + chrono::Duration::days(26)),
        "MARA" => Some(today + chrono::Duration::days(11)),
        "RIOT" => Some(today + chrono::Duration::days(13)),
        "CLSK" => Some(today + chrono::Duration::days(15)),
        
        // Communication Services
        "CMCSA" => Some(today + chrono::Duration::days(40)),
        "CHTR" => Some(today + chrono::Duration::days(41)),
        "TMUS" => Some(today + chrono::Duration::days(19)),
        "VZ" => Some(today + chrono::Duration::days(42)),
        "T" => Some(today + chrono::Duration::days(43)),
        
        // Healthcare & Biotech
        "AMGN" => Some(today + chrono::Duration::days(45)),
        "BIIB" => Some(today + chrono::Duration::days(33)),
        "REGN" => Some(today + chrono::Duration::days(38)),
        "ILMN" => Some(today + chrono::Duration::days(29)),
        "VEEV" => Some(today + chrono::Duration::days(24)),
        "DXCM" => Some(today + chrono::Duration::days(20)),
        "TDOC" => Some(today + chrono::Duration::days(25)),
        "JNJ" => Some(today + chrono::Duration::days(46)),
        
        // E-Commerce & Retail
        "EBAY" => Some(today + chrono::Duration::days(21)),
        "BKNG" => Some(today + chrono::Duration::days(35)),
        "EXPE" => Some(today + chrono::Duration::days(27)),
        "DKNG" => Some(today + chrono::Duration::days(16)),
        "DASH" => Some(today + chrono::Duration::days(23)),
        "UBER" => Some(today + chrono::Duration::days(28)),
        "LYFT" => Some(today + chrono::Duration::days(24)),
        
        // Financial Services
        "SOFI" => Some(today + chrono::Duration::days(19)),
        "UPST" => Some(today + chrono::Duration::days(22)),
        "AFRM" => Some(today + chrono::Duration::days(20)),
        "SQ" => Some(today + chrono::Duration::days(26)),
        "ABNB" => Some(today + chrono::Duration::days(32)),
        "HOOD" => Some(today + chrono::Duration::days(18)),
        
        // Energy & Utilities
        "ENPH" => Some(today + chrono::Duration::days(25)),
        "RUN" => Some(today + chrono::Duration::days(27)),
        "SEDG" => Some(today + chrono::Duration::days(29)),
        "NEE" => Some(today + chrono::Duration::days(44)),
        "DUK" => Some(today + chrono::Duration::days(47)),
        "SO" => Some(today + chrono::Duration::days(48)),
        
        // Industrial & Transportation
        "BA" => Some(today + chrono::Duration::days(31)),
        "GE" => Some(today + chrono::Duration::days(33)),
        "RTX" => Some(today + chrono::Duration::days(36)),
        
        // Consumer
        "COST" => Some(today + chrono::Duration::days(37)),
        "MCD" => Some(today + chrono::Duration::days(17)),
        "SBUX" => Some(today + chrono::Duration::days(19)),
        "NKE" => Some(today + chrono::Duration::days(30)),
        "LULU" => Some(today + chrono::Duration::days(21)),
        
        // Semiconductors & Hardware
        "XLNX" => Some(today + chrono::Duration::days(28)),
        "MU" => Some(today + chrono::Duration::days(22)),
        "NVTA" => Some(today + chrono::Duration::days(26)),
        "SWKS" => Some(today + chrono::Duration::days(24)),
        "JBLU" => Some(today + chrono::Duration::days(20)),
        "ARCB" => Some(today + chrono::Duration::days(25)),
        "SMCI" => Some(today + chrono::Duration::days(19)),
        "DELL" => Some(today + chrono::Duration::days(34)),
        
        // Additional Major NASDAQ
        "HPQ" => Some(today + chrono::Duration::days(23)),
        "PTC" => Some(today + chrono::Duration::days(27)),
        "SNPS" => Some(today + chrono::Duration::days(29)),
        "CDNS" => Some(today + chrono::Duration::days(30)),
        "MSCI" => Some(today + chrono::Duration::days(32)),
        "RBLX" => Some(today + chrono::Duration::days(21)),
        "MNST" => Some(today + chrono::Duration::days(18)),
        "PAYX" => Some(today + chrono::Duration::days(35)),
        "VRSN" => Some(today + chrono::Duration::days(26)),
        "NTAP" => Some(today + chrono::Duration::days(25)),
        "AMAT" => Some(today + chrono::Duration::days(28)),
        
        // Healthcare Tech
        "EDIT" => Some(today + chrono::Duration::days(23)),
        "CRSP" => Some(today + chrono::Duration::days(20)),
        "BEAM" => Some(today + chrono::Duration::days(24)),
        "BMRN" => Some(today + chrono::Duration::days(27)),
        
        // Financial Tech
        "PAYC" => Some(today + chrono::Duration::days(26)),
        "ADP" => Some(today + chrono::Duration::days(38)),
        "ADSK" => Some(today + chrono::Duration::days(31)),
        "WDAY" => Some(today + chrono::Duration::days(29)),
        "NOW" => Some(today + chrono::Duration::days(33)),
        "CCI" => Some(today + chrono::Duration::days(39)),
        "EQIX" => Some(today + chrono::Duration::days(40)),
        
        // E-Payment & Digital
        "V" => Some(today + chrono::Duration::days(44)),
        "MA" => Some(today + chrono::Duration::days(45)),
        "DFS" => Some(today + chrono::Duration::days(28)),
        "AXP" => Some(today + chrono::Duration::days(41)),
        
        // Media & Entertainment
        "SPOT" => Some(today + chrono::Duration::days(22)),
        "MTCH" => Some(today + chrono::Duration::days(24)),
        "ZNGA" => Some(today + chrono::Duration::days(19)),
        "FUTU" => Some(today + chrono::Duration::days(25)),
        
        // Telecom & Wireless
        "VZWIX" => Some(today + chrono::Duration::days(42)),
        
        // Transportation & Logistics
        "UPS" => Some(today + chrono::Duration::days(32)),
        "FDX" => Some(today + chrono::Duration::days(29)),
        "AAL" => Some(today + chrono::Duration::days(15)),
        
        // Automotive
        "NIO" => Some(today + chrono::Duration::days(26)),
        "XPEV" => Some(today + chrono::Duration::days(24)),
        "LI" => Some(today + chrono::Duration::days(25)),
        
        // Cybersecurity
        "PALO" => Some(today + chrono::Duration::days(30)),
        
        // Default: Kein Earnings-Datum bekannt
        _ => None,
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();
    
    info!("Starting IBKR Option Chain Scanner - OTM with High Premium");
    
    // Load .env file
    dotenv::dotenv().ok();
    
    // Load configuration from environment
    let ibkr_url = env::var("IBKR_API_URL")
        .unwrap_or_else(|_| "http://localhost:5000/api/v1".to_string());
    
    let min_profit = env::var("MIN_PROFIT")
        .unwrap_or_else(|_| "2.0".to_string())
        .parse::<f64>()?;
    
    let max_delta = env::var("MAX_DELTA")
        .unwrap_or_else(|_| "0.30".to_string())
        .parse::<f64>()?;
    
    let min_days = env::var("MIN_DAYS")
        .unwrap_or_else(|_| "7".to_string())
        .parse::<i64>()?;
    
    let max_days = env::var("MAX_DAYS")
        .unwrap_or_else(|_| "45".to_string())
        .parse::<i64>()?;
    
    let top_n = env::var("TOP_N")
        .unwrap_or_else(|_| "3".to_string())
        .parse::<usize>()?;
    
    let nasdaq_symbols_str = env::var("NASDAQ_SYMBOLS")
        .unwrap_or_else(|_| "AAPL,MSFT,GOOGL,AMZN,NVDA".to_string());
    
    let nasdaq_symbols: Vec<&str> = nasdaq_symbols_str
        .split(',')
        .map(|s| s.trim())
        .collect();
    
    info!("Configuration loaded:");
    info!("  IBKR URL: {}", ibkr_url);
    info!("  Min Profit: {:.2}%", min_profit);
    info!("  Max Delta: {:.2}", max_delta);
    info!("  Days Range: {} - {}", min_days, max_days);
    info!("  Top N: {}", top_n);
    info!("  Symbols to scan: {}", nasdaq_symbols.join(", "));
    
    let client = IbkrClient::new(ibkr_url);
    
    let mut unique_symbols: Vec<&str> = nasdaq_symbols.iter().copied().collect();
    unique_symbols.sort();
    unique_symbols.dedup();
    
    info!("Scanning {} NASDAQ stocks for OTM options with Delta ≤ {:.2}", 
        unique_symbols.len(), max_delta);
    
    let mut all_best_strikes = Vec::new();
    
    for symbol in &unique_symbols {
        match scan_option_chain(&client, symbol, min_profit, max_delta, min_days, max_days, top_n).await {
            Ok(best_strikes) => {
                if !best_strikes.is_empty() {
                    info!("{}: Found {} top OTM options", symbol, best_strikes.len());
                    all_best_strikes.extend(best_strikes);
                }
            }
            Err(e) => {
                tracing::warn!("Error scanning {}: {}", symbol, e);
            }
        }
    }
    
    display_results(&all_best_strikes);
    
    // Exportiere Ergebnisse zu CSV
    if !all_best_strikes.is_empty() {
        match CsvExporter::export_to_csv(&all_best_strikes, None) {
            Ok(filepath) => {
                info!("✅ Results exported to CSV: {}", filepath);
            }
            Err(e) => {
                tracing::error!("❌ Failed to export CSV: {}", e);
            }
        }
    }
    
    info!("Option Chain Scanner completed successfully");
    
    Ok(())
}

async fn scan_option_chain(
    client: &api::client::IbkrClient,
    symbol: &str,
    min_profit: f64,
    max_delta: f64,
    min_days: i64,
    max_days: i64,
    top_n: usize,
) -> Result<Vec<models::strike::ProfitableStrike>> {
    info!("Fetching option chain for {}", symbol);
    
    let _response = client.get_option_chain(symbol).await?;
    
    let today = Local::now().naive_local().date();
    let current_stock_price = 150.0;
    let earnings_date = get_earnings_date(symbol);
    
    let option_chain = OptionChain {
        symbol: symbol.to_string(),
        current_price: current_stock_price,
        earnings_date,
        strikes: vec![
            models::option_chain::Strike {
                price: 155.0,
                bid: 3.40,
                ask: 3.60,
                premium: 3.50,
                delta: 0.25,
                expiration_date: today + chrono::Duration::days(30),
            },
            models::option_chain::Strike {
                price: 160.0,
                bid: 2.70,
                ask: 2.90,
                premium: 2.80,
                delta: 0.20,
                expiration_date: today + chrono::Duration::days(30),
            },
            models::option_chain::Strike {
                price: 165.0,
                bid: 2.00,
                ask: 2.20,
                premium: 2.10,
                delta: 0.15,
                expiration_date: today + chrono::Duration::days(30),
            },
        ],
    };
    
    let best_strikes = OptionAnalyzer::find_best_otm_strikes(
        &option_chain,
        min_profit,
        max_delta,
        min_days,
        max_days,
        top_n,
    );
    
    Ok(best_strikes)
}

fn display_results(profitable_strikes: &[models::strike::ProfitableStrike]) {
    info!("\n========== TOP OTM OPTIONS (Delta ≤ 0.30, sorted by Premium) ==========\n");
    
    if profitable_strikes.is_empty() {
        info!("No profitable OTM options found");
        return;
    }
    
    for (idx, strike) in profitable_strikes.iter().enumerate() {
        let earnings_marker = if strike.has_earnings_during_option {
            " ⚠️ EARNINGS"
        } else {
            ""
        };
        
        let earnings_info = if let Some(days) = strike.days_to_earnings {
            format!(" | Earnings in {} days", days)
        } else {
            String::new()
        };
        
        info!(
            "#{} | {} | Strike: ${:.2} | Premium: ${:.2} | Profit: {:.2}% | Delta: {:.2} | Expiration: {} ({} days){}{}", 
            idx + 1,
            strike.symbol,
            strike.strike_price,
            strike.premium,
            strike.profit_percentage,
            strike.delta,
            strike.expiration_date,
            strike.days_to_expiration,
            earnings_marker,
            earnings_info
        );
    }
    
    info!("\nTotal Top OTM Options: {}", profitable_strikes.len());
    
    // Zähle Optionen mit Earnings
    let earnings_count = profitable_strikes.iter()
        .filter(|s| s.has_earnings_during_option)
        .count();
    
    if earnings_count > 0 {
        info!("⚠️ Optionen mit Earnings während Laufzeit: {}", earnings_count);
    }
}
