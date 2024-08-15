use crate::handler::handle_recived_data;
use tokio::net::TcpStream;
use Redox::schema::config::get_config;
use Redox::utils::active_internet_connection;
use Redox::utils::network::receive;

pub async fn start_connector() {
    let mut is_connected;
    loop {
        is_connected = active_internet_connection().await;
        // println!("Checking for internet connection:::: {}", is_connected);
        if is_connected {
            break;
        }
        println!("Rechecking conn")
    }
    println!("Internet connection established");
    let config = get_config().await;
    println!("Connecting to the server {}, {}", config.get_host(), config.get_port());
    let mut stream = match TcpStream::connect(format!("{}:{}", config.get_host(), config.get_port())).await {
        Ok(stream) => stream,
        Err(_) => {
            println!("Failed to connect to the server");
            return;
        }
    };
    loop {
        let r = receive(&mut stream).await.unwrap();
        handle_recived_data(&mut stream, r).await;
        
    }
}

