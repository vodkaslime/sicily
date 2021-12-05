use bytes::BytesMut;
use log;
use num::BigUint;
use std::sync::Arc;
use tokio::io::AsyncReadExt;
use tokio::net::TcpListener;

use crate::command::{ Request };
use crate::config::Config;
use crate::location::Location;
use crate::node::NodeList;
use crate::result::Result;

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
                match parse_request(&buf, node_list.clone()) {
                    Ok(request) => {
                        log::info!("The request is: {:?}", request);
                    },
                    Err(err) => {
                        log::error!("Could not process request: {}", err);
                    }
                };
            }
        });
    }
}

fn parse_request(buf: &BytesMut, node_list: Arc<NodeList>) -> Result<Request> {
    let mut s = String::from_utf8(buf.to_vec())?;

    /* Remove trailing "\r\n" if the command if input by user. */
    if s.ends_with("\r\n") {
        s = s[0..s.len()-2].to_string();
    }
    let arr: Vec<&str> = s.split(" ").collect();

    if arr.len() <= 1 {
        return Err(
            "Invalid command. More than one parameter required."
            .into());
    }
    let command = match arr[0].to_lowercase().as_str() {
        "lookup" => {
            if arr.len() != 3 {
                return Err(
                    "Invalid command. Lookup command takes 2 parameters."
                    .into());
            }

            let virtual_node_id = str::parse::<u8>(arr[1])?;
            if virtual_node_id as usize >= node_list.node_list.len() {
                return Err(
                    "Invalid command. Virtual node number too large."
                    .into());
            }

            let key;
            match BigUint::parse_bytes(arr[2].as_bytes(), 16) {
                Some(k) => {
                    key = k;
                },
                None => {
                    return Err(
                        "Invalid command. Failed to parse identifier."
                        .into()
                    );
                }
            };

            log::debug!("The arr[2] is: {}", arr[2]);
            log::debug!("The arr[2] is: {:?}", arr[2].as_bytes());

            Request::Lookup {
                virtual_node_id,
                key
            }
        },

        "join" => {
            if arr.len() != 3 {
                return Err(
                    "Invalid command. Lookup command takes 2 parameters."
                    .into());
            }

            let virtual_node_id = str::parse::<u8>(arr[1])?;
            Request::Join {
                virtual_node_id,
                location: Location::from_string(arr[2].to_string())?,
            }
        }
        _ => {
            return Err(
                "Invalid command. Unrecognized command."
                .into());
        }
    };
    Ok(command)
}