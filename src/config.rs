#[derive(Debug, Clone)]
pub struct Config {
    host: String,
    port: u16,
}

impl Config {
    pub fn new(host: String, port: u16) -> Self {
        Self {
            host,
            port,
        }
    }

    pub fn url(&self) -> String {
        format!("{}:{}", self.host.clone(), self.port.clone())
    }

    pub fn get_host(&self) -> &String { &self.host }
    pub fn get_port(&self) -> &u16 { &self.port }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            host: "127.0.0.1".to_string(),
            port: 8000,
        }
    } 
}
