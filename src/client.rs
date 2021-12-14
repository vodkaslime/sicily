use bytes::BytesMut;
use std::sync::Arc;
use tokio::io::{ AsyncReadExt, AsyncWriteExt };
use tokio::net::{ TcpStream };

use crate::command::{ Request, Response };
use crate::config::Config;
use crate::constants::*;
use crate::location::Location;
use crate::utils::Result;

pub struct Client {
    pub socket: TcpStream,
    pub buffer: BytesMut,
}

impl Client {
    pub async fn new(location: &Location) -> Result<Self> {
        let addr = location.to_addr()?;
        let socket = TcpStream::connect(addr).await?;
        let buffer = BytesMut::with_capacity(OUTPUT_BUFFER_SIZE);
        let client = Self {
            socket,
            buffer,
        };
        Ok(client)
    }

    pub async fn send_request(&mut self, request: Request) -> Result<()> {
        let req_string = request.serialize()?;
        self.socket.write_all(req_string.as_bytes()).await?;
        self.socket.flush().await?;
        Ok(())
    }

    pub async fn receive(&mut self, config: Arc<Config>) -> Result<Response> {
        let n = self.socket.read_buf(&mut self.buffer).await?;
        if n == 0 {
            return Err("[Client side] Error receiving response. Server side closed the connection.".into());
        }
        let response = Response::parse_from_buf(&self.buffer, config)?;
        Ok(response)
    }
}