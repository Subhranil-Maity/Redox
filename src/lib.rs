
/*
Send Sequence:
1. send datatype along with size
2. send data 
 */
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;

/*
text: 1
 */

pub struct ReturnedData {
    pub datatype: String,
    pub data: Vec<u8>,
}

pub async fn send_text(socket: &mut TcpStream, message: &str) -> Result<(), Box<dyn std::error::Error>> {
    let message_bytes = message.as_bytes();
    let length = message_bytes.len();
    let payload1 = format!("1:{}", length);
    let payload1_bytes = payload1.as_bytes();
    let mut p1 = vec![0u8; 1024];
    p1[0..payload1_bytes.len()].copy_from_slice(payload1_bytes);
    // println!("Sending data: {:?}", p1);
    // p1.copy_from_slice(payload1_bytes);
    socket.write(&p1).await?;
    socket.write(message_bytes).await?;
    Ok(())
}


pub async fn receive(socket: &mut TcpStream) -> Result<ReturnedData, Box<dyn std::error::Error>> {
    let mut buf = vec![0; 1024];
    let mut n = socket.read(&mut buf).await?;
    let data = String::from_utf8(buf[0..n].to_vec())?;
    println!("Received data: {}", data);
    // exit(0);
    let data = data.split(":").collect::<Vec<&str>>();
    let datatype = data[0];
    // print!("{:#?}", data);
    let size = data[1].replace('\0', "").trim().parse::<usize>()?;
    // let size = 50;
    let mut buf = vec![0; size];
    n = socket.read(&mut buf).await?;
    // assert_eq!(n, size);
    Ok(ReturnedData {
        datatype: datatype.to_string(),
        data: buf,
    })
}