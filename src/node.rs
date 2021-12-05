use num::bigint::BigUint;
use tokio::sync::Mutex;

use crate::arithmetic;
use crate::config::Config;
use crate::location::Location;

#[derive(Debug)]
pub struct Node {
    location: Location,
    predecessor: Location,
    successor: Location,
    finger: Vec<Location>,
}

impl Node {
    pub fn new(
        config: &Config,
        virtual_node_id: u8,
    ) -> Self {
        let location = Location::new(config, virtual_node_id);
        let predecessor = location.clone();
        let successor = location.clone();

        let mut finger: Vec<Location> = Vec::new();
        for _ in 0..config.id_bits {
            finger.push(location.clone());
        }

        Self {
            location,
            predecessor,
            successor,
            finger,
        }
    }

    pub fn closest_preceding_finger(&self, id: BigUint) -> Location {
        for i in (0..self.finger.len()).rev() {
            let location = &self.finger[i];

            if arithmetic::is_in_range(
                &location.identifier,
                ( &self.location.identifier, false ),
                ( &id, false ),
            ) {
                return location.clone();
            }
        }
        return self.location.clone();
    }
}

#[derive(Debug)]
pub struct NodeList {
    node_list: Vec<Mutex<Node>>,
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