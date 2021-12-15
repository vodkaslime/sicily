/* 
 * This file is part of the Sicily distribution (https://github.com/JeepYiheihou/sicily).
 * Copyright (c) 2021 Jiachen Bai.
 * 
 * This program is free software: you can redistribute it and/or modify  
 * it under the terms of the GNU General Public License as published by  
 * the Free Software Foundation, version 3.
 *
 * This program is distributed in the hope that it will be useful, but 
 * WITHOUT ANY WARRANTY; without even the implied warranty of 
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU 
 * General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License 
 * along with this program. If not, see <http://www.gnu.org/licenses/>.
 */

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