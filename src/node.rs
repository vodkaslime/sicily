use crate::config::Config;
use crate::location::Location;

#[derive(Clone, Debug)]
pub struct Node {
    location: Location,
    predecessor: Location,
    successor: Location,
    finger: Vec<Option<Location>>,
}

impl Node {
    pub fn new(
        config: &Config,
        virtual_node_id: u8,
    ) -> Self {
        let location = Location::new(config, virtual_node_id);
        let predecessor = location.clone();
        let successor = location.clone();

        let mut finger: Vec<Option<Location>> = Vec::new();
        for _ in 0..config.id_bits {
            finger.push(None);
        }

        Self {
            location,
            predecessor,
            successor,
            finger,
        }
    }
}

#[derive(Clone, Debug)]
pub struct NodeList {
    node_list: Vec<Option<Node>>,
}

impl NodeList {
    pub fn new(config: &Config) -> Self {
        let mut node_list: Vec<Option<Node>> = Vec::new();
        for i in 0..config.virtual_node_number {
            node_list.push(Some(Node::new(config, i)));
        }

        Self {
            node_list,
        }
    }
}