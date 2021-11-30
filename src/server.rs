use bytes::BytesMut;
use tokio::io::AsyncReadExt;
use tokio::net::TcpListener;

use crate::result::Result;

#[tokio::main]
pub async fn start() -> Result<()>{
    let port: u16 = 8820;
    let listener = TcpListener::bind(&format!("127.0.0.1:{}", port)).await?;
    println!("Listening to port: {}", port);
    loop {
        match listener.accept().await {
            Ok((mut stream, _)) => {
                println!("A new client connected.");
                tokio::spawn(async move {
                    loop {
                        let mut buf = BytesMut::with_capacity(1024);
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