use crate::utils::{active_internet_connection, get_config};
use tokio::net::TcpStream;
use Redox::receive;


pub async fn start_connector() {
    let mut is_connected;
    loop {
        is_connected = active_internet_connection().await;
        // println!("Checking for internet connection:::: {}", is_connected);
        if is_connected {
            break;
        }
    }
    println!("Internet connection established");
    let config = get_config().await;
    println!("Connecting to the server {}, {}", config.get_host(), config.get_port());
    let mut stream = match TcpStream::connect(format!("{}:{}", config.get_host(), config.get_port())).await{
        Ok(stream) => stream,
        Err(_) => {
            println!("Failed to connect to the server");
            return;
        }
    };
    let data = receive(&mut stream).await.unwrap();
    if data.datatype.eq("1") { 
        println!("Received data: {}", String::from_utf8(data.data).unwrap());
    }
    
}