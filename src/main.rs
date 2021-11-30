pub mod result;
pub mod server;

fn main() -> result::Result<()>{
    server::start()
}
