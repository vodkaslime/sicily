use num::BigUint;

use crate::location::Location;

#[derive(Debug)]
pub enum Request {
    Lookup {
        virtual_node_id: u8,
        key: BigUint,
    },
    Join {
        virtual_node_id: u8,
        location: Location,
    },
    GetSuccessor {
        virtual_node_id: u8,
    }
}

#[derive(Debug)]
pub enum Response {
    Lookup {
        location: Location,
    },
    Join {
        location: Location,
    },
}