use num::bigint::BigUint;
use tokio::sync::Mutex;

use crate::arithmetic;
use crate::config::Config;
use crate::location::Location;
use crate::utils::Result;

#[derive(Debug)]
pub struct Node {
    location: Location,
    predecessor: Option<Location>,
    finger: Vec<Option<Location>>,
    finger_start_identifier: Vec<BigUint>,
}

/*
 * I want to make the program as simple as possible, so I prefer direct access
 * to structs. However for Node, there are some tricky places, for example the
 * "successor" is actually the finger[0], accroding to the paper.
 * 
 * Therefore the Node struct is well wrapped and we need to use getter and setter
 * to operate on it.
 */
impl Node {
    pub fn new(
        config: &Config,
        virtual_node_id: u8,
    ) -> Self {
        let location = Location::new(config, virtual_node_id);
        let predecessor = Some(location.clone());

        let mut finger: Vec<Option<Location>> = Vec::new();
        let mut finger_start_identifier: Vec<BigUint> = Vec::new();
        for i in 0..config.id_bits {
            /* Initialize finger list with local location. */
            finger.push(Some(location.clone()));

            /* Initialize finger start index list. */
            let identifier = &location.identifier;
            let base = BigUint::from_bytes_be(&[2]);
            let pow = base.pow(i as u32);
            let divisor = base.pow(config.id_bits as u32);
            let start_index = (identifier + pow) % divisor;
            finger_start_identifier.push(start_index);
        }

        Self {
            location,
            predecessor,
            finger,
            finger_start_identifier,
        }
    }

    pub fn own_location(&self) -> Location {
        self.location.clone()
    }

    /* The  */
    pub fn get_successor(&self) -> Result<Location> {
        let successor = Location::option_to_result(&self.finger[0])?;
        Ok(successor)
    }

    pub fn set_successor(&mut self, successor: Option<Location>) {
        self.finger[0] = successor;
    }

    pub fn get_predecessor(&self) -> Result<Location> {
        let predecessor = Location::option_to_result(&self.predecessor)?;
        Ok(predecessor)
    }

    pub fn set_predecessor(&mut self, predecessor: Option<Location>) {
        self.predecessor = predecessor;
    }

    pub fn get_finger(&self, n: usize) -> Result<Location> {
        validate_index(&self.finger, n)?;
        let location = Location::option_to_result(&self.finger[n])?;
        Ok(location)
    }

    pub fn set_finger(&mut self, n: usize, location: Option<Location>) -> Result<()> {
        validate_index(&self.finger, n)?;
        self.finger[n] = location;
        Ok(())
    }

    pub fn get_finger_len(&self) -> usize {
        self.finger.len()
    }

    pub fn get_finger_start_identifier(&self, n: usize) -> Result<BigUint> {
        validate_index(&self.finger_start_identifier, n)?;
        let identifier = self.finger_start_identifier[n].clone();
        Ok(identifier)
    }

    pub fn closest_preceding_finger(&self, id: BigUint) -> Result<Location> {
        for i in (0..self.finger.len()).rev() {
            let location = Location::option_to_result(&self.finger[i])?;

            if arithmetic::is_in_range(
                &location.identifier,
                ( &self.location.identifier, false ),
                ( &id, false ),
            ) {
                return Ok(location);
            }
        }
        Ok(self.location.clone())
    }

    pub fn notify_with(&mut self, notifier: &Location) {
        /* The flag to see if the current node needs to update predecessor. */
        let flag = match &self.predecessor {
            Some(predecessor) => {
                arithmetic::is_in_range(
                    &notifier.identifier,
                    (&predecessor.identifier, false),
                    (&self.location.identifier, false))
            },
            None => true,
        };
        if flag {
            self.predecessor = Some(notifier.clone());
        }
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

fn validate_index<T>(vec: &Vec<T>, n: usize) -> Result<()> {
    if n >= vec.len() {
        return Err("Error retrieving finger. Index overflow.".into());
    }
    Ok(())
}