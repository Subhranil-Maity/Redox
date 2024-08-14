use crate::config::Config;
use crate::myconst::CONFIG_URL;
use std::time::{SystemTime, UNIX_EPOCH};

pub fn get_current_timestamp() -> String {
    let start = SystemTime::now();
    let since_the_epoch = start.duration_since(UNIX_EPOCH)
        .expect("Time went backwards");
    format!("{}", since_the_epoch.as_secs())
}

pub async fn active_internet_connection() -> bool {
    // let test_address = "8.8.8.8:53"; // Google's public DNS server
    // let socket = UdpSocket::bind("127.0.0.1:34254").expect("Failed to bind address");
    // socket.set_read_timeout(Some(Duration::from_secs(2))).expect("Failed to set read timeout");
    // // println!("some");
    // socket.connect(test_address).is_ok()
    
    match reqwest::get("https://api.ipify.org/").await {
        Ok(_) => true,
        Err(_) => false
    }
}

pub async fn get_config() -> Config{
    let config = reqwest::get(CONFIG_URL).await.unwrap().json::<Config>().await.unwrap();
    config
}

