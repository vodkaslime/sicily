
pub mod config;
pub mod constants;
pub mod result;
pub mod server;

fn main() -> result::Result<()>{
    let config = config::parse_params()?;
    println!("The config is {:?}", config);
    server::start(&config)
}
