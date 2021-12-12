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
pub async fn start(config: &Config, node_list: Arc<NodeList>) -> Result<()> {
    let port = config.port;
    let output_buffer_size = config.output_buffer_size;
    let handle = tokio::spawn(async move {
        start_core_loop(port, output_buffer_size, node_list.clone()).await
    });
    handle.await?;
    Ok(())
}

async fn start_core_loop(
    port: u16,
    output_buffer_size: usize,
    node_list: Arc<NodeList>,
) {
    let listener = match TcpListener::bind(&format!("127.0.0.1:{}", port)).await {
        Ok(listener) => { listener }
        Err(e) => {
            log::error!("Error initializing listener. Error log: {}", e);
            return;
        }
    };
    log::info!("Listening to port: {}", port);
    loop {
        let node_list = node_list.clone();
        match listener.accept().await {
            Ok((mut stream, _)) => { 
                log::info!("A new client connected.");
                tokio::spawn(async move {
                    let mut buf = BytesMut::with_capacity(output_buffer_size);
                    loop {
                        match stream.read_buf(&mut buf).await {
                            Ok(n) => {
                                if n == 0 {
                                    log::info!("Client disconnected.");
                                    return;
                                }
        
                                match command::process_request(&buf, node_list.clone()).await {
                                    Ok(string) => {
                                        write_to_socket(&mut stream, string).await;
                                        buf.clear();
                                    },
                                    Err(err) => {
                                        log::error!("Could not process request: {}", err);
                                        return;
                                    }
                                };
                            },
                            Err(e) => {
                                log::error!("Error reading from socket: {}", e);
                                return;
                            }
                        }
                    }
                });
            }
            Err(e) => {
                log::error!("Error accepting. Error log: {}", e);
            } 
        };
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