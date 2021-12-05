use bytes::BytesMut;
use log;
use std::sync::Arc;
use tokio::io::{ AsyncReadExt, AsyncWriteExt };
use tokio::net::{ TcpListener, TcpStream };

use crate::command;
use crate::config::Config;
use crate::node::NodeList;
use crate::utils::Result;

#[tokio::main]
pub async fn start(config: &Config, node_list: Arc<NodeList>) -> Result<()>{
    let listener = TcpListener::bind(&format!("127.0.0.1:{}", config.port)).await?;
    log::info!("Listening to port: {}", config.port);
    let input_buffer_size = config.input_buffer_size;
    loop {
        let node_list = node_list.clone();
        let (mut stream, _) = listener.accept().await?;
        log::info!("A new client connected.");
        tokio::spawn(async move {
            loop {
                let mut buf = BytesMut::with_capacity(input_buffer_size);
                let n = stream.read_buf(&mut buf).await.unwrap();
                if n == 0 {
                    log::info!("Client disconnected.");
                    return;
                }
                match command::process_request(&buf, node_list.clone()).await {
                    Ok(string) => {
                        write_to_socket(&mut stream, string).await;
                    },
                    Err(err) => {
                        log::error!("Could not process request: {}", err);
                    }
                };
            }
        });
    }
}

async fn write_to_socket(stream: &mut TcpStream, string: String) {
    match stream.write_all(string.as_bytes()).await {
        Ok(_) => {},
        Err(err) => {
            log::error!("Error when writing repsonse to buffer: {}", err);
        }
    }
}