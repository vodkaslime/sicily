extern crate sicily;

use pretty_env_logger;
use std::sync::Arc;

use sicily::utils;
use sicily::node;
use sicily::config;
use sicily::network;

fn main() -> utils::Result<()>{
    pretty_env_logger::init();
    let config = config::parse_params()?;
    let node_list = Arc::new(node::NodeList::new(&config));
    network::start(&config, node_list)
}
