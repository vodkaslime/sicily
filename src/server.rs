use bytes::BytesMut;
use tokio::io::AsyncReadExt;
use tokio::net::TcpListener;

use crate::config::Config;
use crate::result::Result;

#[tokio::main]
pub async fn start(config: &Config) -> Result<()>{
    let _listener = TcpListener::bind(&format!("127.0.0.1:{}", config.port)).await?;
    println!("Listening to port: {}", config.port);
    let input_buffer_size = config.input_buffer_size;
    loop {
        match _listener.accept().await {
            Ok((mut stream, _)) => {
                println!("A new client connected.");
                tokio::spawn(async move {
                    loop {
                        let mut buf = BytesMut::with_capacity(input_buffer_size);
                        let n = stream.read_buf(&mut buf).await.unwrap();
                        if n == 0 {
                            println!("Client disconnected.");
                            return;
                        }
                        println!("Got from client: {:?}", &buf);
                    }
                });
            },
            Err(err) => {
                return Err(err.into());
            }
        }
    }
}