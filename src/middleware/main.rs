use tokio::net::{TcpListener, TcpStream};
use tokio::spawn;
use Redox::send_text;

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
    // let mut buf = vec![0; 1024];
    send_text(socket, "Hello World!").await.unwrap();
}