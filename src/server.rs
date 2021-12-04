use bytes::BytesMut;
use log;
use tokio::io::AsyncReadExt;
use tokio::net::TcpListener;

use crate::command::{ Request, Response };
use crate::config::Config;
use crate::location::Location;
use crate::node::NodeList;
use crate::result::Result;

#[tokio::main]
pub async fn start(config: &Config, node_list: NodeList) -> Result<()>{
    let listener = TcpListener::bind(&format!("127.0.0.1:{}", config.port)).await?;
    log::info!("Listening to port: {}", config.port);
    let input_buffer_size = config.input_buffer_size;
    loop {
        match listener.accept().await {
            Ok((mut stream, _)) => {
                log::info!("A new client connected.");
                tokio::spawn(async move {
                    loop {
                        let mut buf = BytesMut::with_capacity(input_buffer_size);
                        let n = stream.read_buf(&mut buf).await.unwrap();
                        if n == 0 {
                            log::info!("Client disconnected.");
                            return;
                        }
                        match process_request(&buf) {
                            Ok(()) => {},
                            Err(err) => {
                                log::error!("Could not process request: {}", err);
                            }
                        };
                    }
                });
            },
            Err(err) => {
                return Err(err.into());
            }
        }
    }
}

/*
 * Process a request from buffer, and parse them into
 * command::Request to execute.
 */
fn process_request(buf: &BytesMut) -> Result<()> {
    let mut s = String::from_utf8(buf.to_vec())?;

    /* Remove trailing "\r\n" if the command if input by user. */
    if s.ends_with("\r\n") {
        s = s[0..s.len()-2].to_string();
    }
    let arr: Vec<&str> = s.split(" ").collect();

    if arr.len() <= 1 {
        return Err("Invalid command. More than one parameter required.".into());
    }
    let command = match arr[0] {
        "GET" => {
            Request::Get {
                key: arr[1].to_string()
            }
        },

        "JOIN" => {
            Request::Join {
                location: Location::from_string(arr[1].to_string())?,
            }
        }
        _ => {
            return Err("Invalid command.".into());
        }
    };

    log::info!("The command is: {:?}", command);
    Ok(())
}