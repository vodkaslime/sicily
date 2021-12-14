use bytes::BytesMut;
use log;
use std::sync::Arc;
use tokio::io::{ AsyncReadExt, AsyncWriteExt };
use tokio::net::{ TcpListener, TcpStream };
use tokio::task::JoinHandle;

use crate::command;
use crate::config::Config;
use crate::membership;
use crate::node::NodeList;
use crate::utils::Result;

#[tokio::main]
pub async fn start(config: &Config, node_list: Arc<NodeList>) -> Result<()> {
    let port = config.port;
    let output_buffer_size = config.output_buffer_size;
    let node_list_clone = node_list.clone();
    let handle = tokio::spawn(async move {
        start_core_loop(port, output_buffer_size, node_list_clone).await
    });
    let mut stabilizing_handles = start_stabilizing_tasks(config, node_list.clone()).await;
    handle.await?;
    for i in 0..stabilizing_handles.len() {
        let handler = &mut stabilizing_handles[i];
        handler.await?
    }
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
        let stream: TcpStream;

        match listener.accept().await {
            Ok((s, _)) => {
                stream = s
            }
            Err(e) => {
                log::error!("Error accepting. Error log: {}", e);
                continue;
            } 
        };

        tokio::spawn(async move {
            handle_socket_read(stream, output_buffer_size, node_list).await;
        });
    }
}

async fn handle_socket_read(
    mut stream: TcpStream,
    output_buffer_size: usize,
    node_list: Arc<NodeList>
) {
    let mut buf = BytesMut::with_capacity(output_buffer_size);
    loop {
        match stream.read_buf(&mut buf).await {
            Ok(n) => {
                if n == 0 {
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
}

async fn write_to_socket(stream: &mut TcpStream, string: String) {
    match stream.write_all(string.as_bytes()).await {
        Ok(_) => {},
        Err(err) => {
            log::error!("Error when writing repsonse to buffer: {}", err);
        }
    }
}

async fn start_stabilizing_tasks(config: &Config, node_list: Arc<NodeList>) -> Vec<JoinHandle<()>> {
    let mut vec: Vec<JoinHandle<()>> = Vec::new();
    for i in 0..config.virtual_node_number {
        let node_list = node_list.clone();
        let handler = tokio::spawn(async move {
            start_stabilizing_task(i, node_list).await;
        });
        vec.push(handler);
    }
    vec
}

async fn start_stabilizing_task(virtual_node_id: u8, node_list: Arc<NodeList>) {
    loop {
        tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;
        let node_list = node_list.clone();
        match membership::stablize(virtual_node_id, node_list.clone()).await {
            Ok(()) => {
                /* Happy case. Nothing to do. */
            },
            Err(e) => {
                log::error!{"Error stabilizing at virtual node id {}. Error message: {}.", virtual_node_id, e};
            }
        }
        match membership::fix_fingers(virtual_node_id, node_list.clone()).await {
            Ok(()) =>{
                /* Happy case. Nothing to do. */
            },
            Err(e) => {
                log::error!("Error fixing fingers at virtual node id {}. Error message: {}.", virtual_node_id, e);
            }
        }
    }
}