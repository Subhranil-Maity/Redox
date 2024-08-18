use crate::myconst::{CONFIG_URL, OFFLINE_MODE};
use serde::{Deserialize, Serialize};

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
pub async fn get_config() -> Config {
    let config;
    if !OFFLINE_MODE {
        config = reqwest::get(CONFIG_URL).await.unwrap().json::<Config>().await.unwrap();
    } else {
        println!("Running on no Internet mode");
        config = Config {
            host: "127.0.0.1".to_string(),
            port: 3000,
        };
    }
    config
}