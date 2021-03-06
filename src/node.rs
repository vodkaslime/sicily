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

use num::bigint::BigUint;
use std::sync::Arc;
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
        config: Arc<Config>,
        virtual_node_id: u8,
    ) -> Self {
        let location = Location::new(config.clone(), virtual_node_id);
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

    /*
     * Get the location location.
     */
    pub fn own_location(&self) -> Location {
        self.location.clone()
    }

    /*
     * Get the location of successor.
     */
    pub fn get_successor(&self) -> Result<Location> {
        let successor = Location::option_to_result(&self.finger[0])?;
        Ok(successor)
    }

    /*
     * Set the location of successor.
     */
    pub fn set_successor(&mut self, successor: Option<Location>) {
        self.finger[0] = successor;
    }

    /*
     * Get the location of predecessor.
     */
    pub fn get_predecessor(&self) -> Result<Location> {
        let predecessor = Location::option_to_result(&self.predecessor)?;
        Ok(predecessor)
    }

    /*
     * Set the location of the predecessor.
     */
    pub fn set_predecessor(&mut self, predecessor: Option<Location>) {
        self.predecessor = predecessor;
    }

    /*
     * Get the location from finger list at index n.
     */
    pub fn get_finger(&self, n: usize) -> Result<Location> {
        validate_index(&self.finger, n)?;
        let location = Location::option_to_result(&self.finger[n])?;
        Ok(location)
    }

    /*
     * Set the location of finger list at index n.
     */
    pub fn set_finger(&mut self, n: usize, location: Option<Location>) -> Result<()> {
        validate_index(&self.finger, n)?;
        self.finger[n] = location;
        Ok(())
    }

    /*
     * Get the length of the finger list.
     */
    pub fn get_finger_len(&self) -> usize {
        self.finger.len()
    }

    /*
     * Get the identifier from finger start identifier list at index n.
     */
    pub fn get_finger_start_identifier(&self, n: usize) -> Result<BigUint> {
        validate_index(&self.finger_start_identifier, n)?;
        let identifier = self.finger_start_identifier[n].clone();
        Ok(identifier)
    }

    /*
     * Get and format the information of the node.
     */
    pub fn get_info(&self) -> String {
        let mut info = "\r\n".to_string();
        /* My own location. */
        let own_location_string = self.location.to_info();
        info.push_str(&format!("My own location: {}\r\n", own_location_string));

        /* Predecessor. */
        let predecessor_string = Location::print_info_from_option(&self.predecessor);
        info.push_str(&format!("Predecessor: {}\r\n", predecessor_string));

        /* Successor. */
        let successor_string = Location::print_info_from_option(&self.finger[0]);
        info.push_str(&format!("Successor: {}\r\n", successor_string));

        /* Fingers and start index. */
        info.push_str(&format!("The finger list len is: {}\r\n", self.finger.len()));
        for i in 0..self.finger.len() {
            info.push_str(&format!("Finger {}: ", i));
            let finger_string = Location::print_info_from_option(&self.finger[i]);
            let start_identifier = &self.finger_start_identifier[i];
            info.push_str(&format!("{}\r\n", finger_string));
            info.push_str(&format!("{} --> start index\r\n", start_identifier));
        }
        
        info
    }

    /*
     * Get the closest preceding finger of a given key.
     */
    pub fn closest_preceding_finger(&self, key: BigUint) -> Result<Location> {
        for i in (0..self.finger.len()).rev() {
            let location = Location::option_to_result(&self.finger[i])?;

            if arithmetic::is_in_range(
                &location.identifier,
                ( &self.location.identifier, false ),
                ( &key, false ),
            ) {
                return Ok(location);
            }
        }
        Ok(self.location.clone())
    }

    /*
     * Handle the notification at the receiver side:
     * After getting notified with notifier, make a decision whether to
     * mark the notifier as new predecessor.
     */
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
    pub fn new(config: Arc<Config>) -> Self {
        let mut node_list: Vec<Mutex<Node>> = Vec::new();
        for i in 0..config.virtual_node_number {
            let node = Node::new(config.clone(), i);
            node_list.push(Mutex::new(node));
        }

        Self {
            node_list,
        }
    }
}

/*
 * Convenience function to validate whether an index number n is within
 * the capacity of a given vector.
 */
fn validate_index<T>(vec: &Vec<T>, n: usize) -> Result<()> {
    if n >= vec.len() {
        return Err("Error retrieving finger. Index overflow.".into());
    }
    Ok(())
}