use std::net::UdpSocket;
use std::time::{SystemTime, UNIX_EPOCH};
use winapi::um::winuser::GetDC;

pub fn get_current_timestamp() -> String {
    let start = SystemTime::now();
    let since_the_epoch = start.duration_since(UNIX_EPOCH)
        .expect("Time went backwards");
    format!("{}", since_the_epoch.as_secs())
}

pub fn active_internet_connection() -> bool {
    let test_address = "8.8.8.8:53"; // Google's public DNS server
    let socket = UdpSocket::bind("127.0.0.1:34254").expect("Failed to bind address");
    socket.connect(test_address).is_ok()
}

