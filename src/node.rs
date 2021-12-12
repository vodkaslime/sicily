use num::bigint::BigUint;
use tokio::sync::Mutex;

use crate::arithmetic;
use crate::config::Config;
use crate::location::Location;
use crate::utils::Result;

#[derive(Debug)]
pub struct Node {
    pub location: Location,
    pub predecessor: Option<Location>,
    pub successor: Option<Location>,
    pub finger: Vec<Option<Location>>,
}

impl Node {
    pub fn new(
        config: &Config,
        virtual_node_id: u8,
    ) -> Self {
        let location = Location::new(config, virtual_node_id);
        let predecessor = Some(location.clone());
        let successor = Some(location.clone());

        let mut finger: Vec<Option<Location>> = Vec::new();
        for _ in 0..config.id_bits {
            finger.push(Some(location.clone()));
        }

        Self {
            location,
            predecessor,
            successor,
            finger,
        }
    }

    /*
     * Get finger location at index.
     */
    pub fn get_finger(&self, i: usize) -> Result<Location> {
        if i >= self.finger.len() {
            return Err(
                format!("Invalid index when retrieving finger. Trying with index: {}.", i).
                into());
        }
        let location = retrieve_location(&self.finger[i])?;
        Ok(location)
    }

    /*
     * Get successor.
     */
    pub fn get_successor(&self) -> Result<Location> {
        let location = retrieve_location(&self.successor)?;
        Ok(location)
    } 

    pub fn closest_preceding_finger(&self, id: BigUint) -> Result<Location> {
        for i in (0..self.finger.len()).rev() {
            let location = self.get_finger(i)?;

            if arithmetic::is_in_range(
                &location.identifier,
                ( &self.location.identifier, false ),
                ( &id, false ),
            ) {
                return Ok(location.clone());
            }
        }
        Ok(self.location.clone())
    }
}

#[derive(Debug)]
pub struct NodeList {
    pub node_list: Vec<Mutex<Node>>,
}

impl NodeList {
    pub fn new(config: &Config) -> Self {
        let mut node_list: Vec<Mutex<Node>> = Vec::new();
        for i in 0..config.virtual_node_number {
            node_list.push(Mutex::new(Node::new(config, i)));
        }

        Self {
            node_list,
        }
    }
}

/*
 * Convenience function to retrieve Location from Option.
 * This is due to the issue that rust doesn't implement Box<Error> for Option's NoneError.
 * Issue link: https://github.com/rust-lang/rust/issues/46871 
 */
pub fn retrieve_location(option: &Option<Location>) -> Result<Location> {
    match option {
        Some(location) => {
            return Ok(location.clone())
        },
        None => {
            return Err("None error encoutered while trying to get something from Option.".into());
        }
    }
}