use std::time::{SystemTime, UNIX_EPOCH};

pub mod network;
mod fs;
mod errors;
mod screenshots;

pub fn get_current_timestamp() -> String {
    let start = SystemTime::now();
    let since_the_epoch = start.duration_since(UNIX_EPOCH)
        .expect("Time went backwards");
    format!("{}", since_the_epoch.as_secs())
}

pub async fn active_internet_connection() -> bool {
    match reqwest::get("https://api.ipify.org/").await {
        Ok(_) => true,
        Err(_) => false
    }
}
