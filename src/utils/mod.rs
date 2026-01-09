pub mod csv_export;

pub fn format_price(price: f64) -> String {
    format!("{:.2}", price)
}