#[derive(Debug, Clone)]
pub struct Strike {
    pub value: f64,
    pub premium: f64,
}

impl Strike {
    pub fn new(value: f64, premium: f64) -> Self {
        Strike { value, premium }
    }

    pub fn profit_margin(&self) -> f64 {
        if self.premium == 0.0 {
            return 0.0;
        }
        (self.value / self.premium - 1.0) * 100.0
    }
}