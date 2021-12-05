use num::bigint::BigUint;

use crate::config::Config;
use crate::hash;
use crate::result::Result;

#[derive(Clone, Debug)]
pub struct Location {
    pub ip: String,
    pub port: u16,
    pub virtual_node_id: u8,
    pub identifier: BigUint,
}

impl Location {
    pub fn new(
        config: &Config,
        virtual_node_id: u8,
    ) -> Self {
        let ip = config.host.clone();
        let port = config.port;
        let id_input = format!("{}:{}:{}", &ip, port, virtual_node_id);
        let identifier = hash::digest(&id_input);
        return Self {
            ip,
            port,
            virtual_node_id,
            identifier,
        }
    }

    pub fn from_string(id_input: String) -> Result<Self> {
        let arr: Vec<&str> = id_input.split(":").collect();
        if arr.len() < 2 || arr.len() > 3 {
            return Err("Invalid number of params for a Join request.".into());
        }

        let ip = arr[0].to_string();
        let port = arr[1].parse::<u16>()?;

        let virtual_node_id;
        if arr.len() == 2 {
            virtual_node_id = 0;
        } else {
            virtual_node_id = arr[2].parse::<u8>()?;
        }
        let identifier = hash::digest(&id_input);
        Ok(Self {
            ip,
            port,
            virtual_node_id,
            identifier,
        })
    }

    pub fn to_string(&self) -> String {
        return format!(
            "{}:{}:{}",
            self.ip,
            self.port,
            self.virtual_node_id
        );
    }
}