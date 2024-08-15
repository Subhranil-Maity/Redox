use serde::{Deserialize, Serialize};
use crate::myconst::CONFIG_URL;

#[derive(Serialize, Deserialize, Clone)]
pub struct Config {
    host: String,
    port: u32,
}

impl Config {
    pub fn new(host: String, port: u32) -> Self {
        Config {
            host,
            port,
        }
    }
    
    pub fn get_host(&self) -> String {
        self.host.clone()
    }
    
    pub fn get_port(&self) -> u32 {
        self.port
    }
}
pub async fn get_config() -> Config{
    let config = reqwest::get(CONFIG_URL).await.unwrap().json::<Config>().await.unwrap();
    config
}