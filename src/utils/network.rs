use crate::ReturnedData;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;

pub async fn send_command(socket: &mut TcpStream, command: &str) -> Result<(), Box<dyn std::error::Error>> {
    let command_bytes = command.as_bytes();
    let length = command_bytes.len();
    let payload1 = format!("2:{}", length);
    let payload1_bytes = payload1.as_bytes();
    let mut p1 = vec![0u8; 1024];
    p1[0..payload1_bytes.len()].copy_from_slice(payload1_bytes);
    socket.write(&p1).await?;
    socket.write(command_bytes).await?;
    Ok(())
}
pub async fn send_text(socket: &mut TcpStream, message: &str) -> Result<(), Box<dyn std::error::Error>> {
    let message_bytes = message.as_bytes();
    let length = message_bytes.len();
    let payload1 = format!("1:{}", length);
    let payload1_bytes = payload1.as_bytes();
    let mut p1 = vec![0u8; 1024];
    p1[0..payload1_bytes.len()].copy_from_slice(payload1_bytes);
    socket.write(&p1).await?;
    socket.write(message_bytes).await?;
    Ok(())
}


pub async fn receive(socket: &mut TcpStream) -> Result<ReturnedData, Box<dyn std::error::Error>> {
    let mut buf = vec![0; 1024];
    socket.read(&mut buf).await?;
    let data = String::from_utf8(buf.to_vec())?;
    if data.replace("\0", "").trim().len() == 0 { 
        return Ok(ReturnedData {
            datatype:"0".to_string(),
            data: vec![],
        })
    }
    println!("Received data: {}", data);
    // exit(0);
    let data = data.split(":").collect::<Vec<&str>>();
    let datatype = data[0];
    
    // print!("{:#?}", data);
    let mut size = 0usize;
    size = data[1].replace('\0', "").trim().parse::<usize>()?;
    // let size = 50;
    let mut buf = vec![0; size];
    socket.read(&mut buf).await?;
    // assert_eq!(n, size);
    Ok(ReturnedData {
        datatype: datatype.to_string(),
        data: buf,
    })
}

pub async fn request_sys_info(socket: &mut TcpStream) -> Result<(), Box<dyn std::error::Error>> {
    let message_bytes = "Sys".as_bytes();
    let length = message_bytes.len();
    let payload1 = format!("3:{}", length);
    let payload1_bytes = payload1.as_bytes();
    let mut p1 = vec![0u8; 1024];
    p1[0..payload1_bytes.len()].copy_from_slice(payload1_bytes);
    socket.write(&p1).await?;
    socket.write(message_bytes).await?;
    Ok(())
}