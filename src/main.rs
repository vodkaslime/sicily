extern crate sicily;

use pretty_env_logger;
use std::sync::Arc;

use sicily::utils;
use sicily::node;
use sicily::config;
use sicily::server;

fn main() -> utils::Result<()>{
    pretty_env_logger::init();
    let config = Arc::new(config::parse_params()?);
    let node_list = Arc::new(node::NodeList::new(config.clone()));
    server::start(node_list, config)?;
    Ok(())
}