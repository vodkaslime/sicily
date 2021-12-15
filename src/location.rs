use num::bigint::BigUint;
use std::net::{ IpAddr, SocketAddr };
use std::str::FromStr;
use std::sync::Arc;

use crate::arithmetic;
use crate::config::Config;
use crate::utils::Result;

#[derive(Clone, Debug)]
pub struct Location {
    pub ip: String,
    pub port: u16,
    pub virtual_node_id: u8,
    pub identifier: BigUint,
}

impl Location {
    pub fn new(config: Arc<Config>, virtual_node_id: u8) -> Self {
        let ip = config.host.clone();
        let port = config.port;
        let id_input = format!("{}:{}:{}", &ip, port, virtual_node_id);
        let identifier = arithmetic::compute_identifier(config.id_bits as u32, &id_input);
        return Self {
            ip,
            port,
            virtual_node_id,
            identifier,
        }
    }

    pub fn from_string(id_input: String, config: Arc<Config>) -> Result<Self> {
        let arr: Vec<&str> = id_input.split(":").collect();
        if arr.len() < 2 || arr.len() > 3 {
            return Err("Invalid number of params for making a location.".into());
        }

        let ip = arr[0].to_string();
        let port = arr[1].parse::<u16>()?;

        let virtual_node_id;
        if arr.len() == 2 {
            virtual_node_id = 0;
        } else {
            virtual_node_id = arr[2].parse::<u8>()?;
        }
        let identifier = arithmetic::compute_identifier(config.id_bits as u32, &id_input);
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

    pub fn to_info(&self) -> String {
        return format!(
            "{}:{}:{}\r\n{} --> identifier",
            self.ip,
            self.port,
            self.virtual_node_id,
            self.identifier
        );
    }

    pub fn to_addr(&self) -> Result<SocketAddr> {
        let addr = SocketAddr::new(IpAddr::from_str(&self.ip)?, self.port);
        Ok(addr)
    }

    /*
     * Convenience function to retrieve Location from Option.
     * This is due to the issue that rust doesn't implement Box<Error> for Option's NoneError.
     * Issue link: https://github.com/rust-lang/rust/issues/46871.
     */
    pub fn option_to_result(option: &Option<Self>) -> Result<Self> {
        match option {
            Some(location) => {
                return Ok(location.clone())
            },
            None => {
                return Err("None error encoutered while trying to get something from Option.".into());
            }
        }
    }

    /*
     * Convenience function to format and return info string of an Option<Location>.
     * If Some(Location) then format and return the location.
     * If None the print "None".
     */
    pub fn print_info_from_option(option: &Option<Self>) -> String {
        match option {
            Some(location) => {
                return location.to_info();
            },
            None => {
                return "None".to_string();
            }
        }
    }
}