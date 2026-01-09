pub struct Gateway {
    host: String,
    port: u16,
}

impl Gateway {
    pub fn new(host: String, port: u16) -> Self {
        Self { host, port }
    }
}