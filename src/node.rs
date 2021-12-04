use crate::config::Config;
use crate::location::Location;

pub struct Node {
    location: Location,
    predecessor: Option<Location>,
    successor: Option<Location>,
    finger: Vec<Option<Location>>,
}

impl Node {
    pub fn new(
        config: &Config,
        virtual_node_id: u8,
    ) -> Self {
        let location = Location::new(config, virtual_node_id);

        let mut finger: Vec<Option<Location>> = Vec::new();
        for _ in 0..config.id_bits {
            finger.push(None);
        }

        return Self {
            location,
            predecessor: None,
            successor: None,
            finger: Vec::new(),
        }
    }
}

pub struct NodeList {
    node_list: Vec<Option<Node>>,
}

impl NodeList {
    pub fn new(config: &Config) -> Self {
        let mut node_list: Vec<Option<Node>> = Vec::new();
        for i in 0..config.virtual_node_number {
            node_list.push(Some(Node::new(config, i)));
        }

        return Self {
            node_list,
        };
    }
}