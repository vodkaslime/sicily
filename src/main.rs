extern crate sicily;

use pretty_env_logger;

use sicily::result;
use sicily::node;
use sicily::config;
use sicily::server;


fn main() -> result::Result<()>{
    pretty_env_logger::init();
    let config = config::parse_params()?;
    let node_list = node::NodeList::new(&config);
    server::start(&config, node_list)
}
