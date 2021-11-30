mod network;

fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>>{
    network::server::start()?;
    Ok(())
}
