/// Application configuration
#[derive(Debug, Clone)]
pub struct Config {
    pub host: String,
    pub port: u16,
    pub json_output: bool,
}

impl Config {
    pub fn new(host: String, port: u16, json_output: bool) -> Self {
        Self {
            host,
            port,
            json_output,
        }
    }

    pub fn base_url(&self) -> String {
        format!("http://{}:{}", self.host, self.port)
    }
}
