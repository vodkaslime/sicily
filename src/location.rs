use crate::config::Config;
use crate::result::Result;

#[derive(Debug)]
pub struct Location {
    pub ip: String,
    pub port: u16,
    pub virtual_node_id: u8,
}

impl Location {
    pub fn new(
        config: &Config,
        virtual_node_id: u8,
    ) -> Self {
        return Self {
            ip: config.host.clone(),
            port: config.port,
            virtual_node_id
        }
    }

    pub fn from_string(s: String) -> Result<Self> {
        let arr: Vec<&str> = s.split(":").collect();
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
        Ok(Self {
            ip,
            port,
            virtual_node_id
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