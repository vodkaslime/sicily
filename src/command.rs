use bytes::BytesMut;
use num::BigUint;
use std::sync::Arc;

use crate::location::Location;
use crate::membership;
use crate::node::NodeList;
use crate::process;
use crate::utils::Result;

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
    },
    GetPredecessor {
        virtual_node_id: u8,
    },
    ClosestPrecedingFinger {
        virtual_node_id: u8,
        key: BigUint,
    },
}

impl Request {

    /*
     * Parse request from buffer.
     * Param node_list is needed here to guard virtual_node_id.
     */
    pub fn parse_from_buf(buf: &BytesMut, node_list: Arc<NodeList>) -> Result<Self> {
        let mut s = String::from_utf8(buf.to_vec())?;

        /* Remove trailing "\r\n" if the command if input by user. */
        if s.ends_with("\r\n") {
            s = s[0..s.len()-2].to_string();
        }
        let arr: Vec<&str> = s.split(" ").collect();

        /* Should have at least one valid string in the array vector after split. */
        if arr.len() <= 1 {
            return Err(
                "Invalid command. More than one parameter required."
                .into());
        }

        /* Start parsing request. */
        let command = match arr[0].to_lowercase().as_str() {
            "lookup" => {
                check_params_len(&arr, 3)?;
                let virtual_node_id = parse_virtual_node_id(arr[1], node_list.clone())?;
                let key = parse_key(arr[2])?;
                Request::Lookup {
                    virtual_node_id,
                    key,
                }
            },

            "join" => {
                check_params_len(&arr, 3)?;
                let virtual_node_id = parse_virtual_node_id(arr[1], node_list.clone())?;
                let location = Location::from_string(arr[2].to_string())?;
                Request::Join {
                    virtual_node_id,
                    location,
                }
            },

            "getsuccessor" => {
                check_params_len(&arr, 2)?;
                let virtual_node_id = parse_virtual_node_id(arr[1], node_list.clone())?;
                Request::GetSuccessor {
                    virtual_node_id,
                }
            },

            "getpredecessor" => {
                check_params_len(&arr, 2)?;
                let virtual_node_id = parse_virtual_node_id(arr[1], node_list.clone())?;
                Request::GetPredecessor {
                    virtual_node_id,
                }
            },

            "closestprecedingfinger" => {
                check_params_len(&arr, 3)?;
                let virtual_node_id = parse_virtual_node_id(arr[1], node_list.clone())?;
                let key = parse_key(arr[2])?;
                Request::ClosestPrecedingFinger {
                    virtual_node_id,
                    key,
                }
            },

            _ => {
                return Err(
                    "Invalid command. Unrecognized command."
                    .into());
            }
        };
        Ok(command)
    }

    pub fn serialize(&self) -> Result<String> {
        let res = match self {
            Request::Lookup { virtual_node_id, key } => {
                format!("LOOKUP {} {}", virtual_node_id, key)
            },
            Request::Join { virtual_node_id, location } => {
                format!("JOIN {} {}", virtual_node_id, location.to_string())
            },
            Request::GetSuccessor { virtual_node_id } => {
                format!("GETSUCCESSOR {}", virtual_node_id)
            },
            Request::GetPredecessor { virtual_node_id } => {
                format!("GETPREDECESSOR {}", virtual_node_id)
            }
            Request::ClosestPrecedingFinger { virtual_node_id, key } => {
                format!("CLOSESTPRECEDINGFINGER {} {}", virtual_node_id, key)
            },
            _ => {
                return Err("Error serializing request. Invalid request type.".into());
            }
        };
        Ok(res)
    }
}

#[derive(Debug)]
pub enum Response {
    Lookup {
        location: Location,
    },
    Join,
    GetSuccessor {
        location: Location,
    },
    GetPredecessor {
        location: Location,
    },
    ClosestPrecedingFinger {
        location: Location,
    },
}

impl Response {
    /*
     * Parse request from buffer.
     * Param node_list is needed here to guard virtual_node_id.
     */
    pub fn parse_from_buf(buf: &BytesMut) -> Result<Self> {
        let s = String::from_utf8(buf.to_vec())?;
        let arr: Vec<&str> = s.split(" ").collect();

        /* Should have at least two valid string in the array vector after split. */
        if arr.len() <= 2 {
            return Err(
                "Invalid response. More than two separate strings required."
                .into());
        }

        if arr[0].to_lowercase() != "res" {
            return Err(
                "Invalid response. The first string is not \"RES\"."
                .into());
        } 

        /* Start parsing response. */
        let response = match arr[1].to_lowercase().as_str() {
            "lookup" => {
                check_params_len(&arr, 3)?;
                let location = Location::from_string(arr[2].to_string())?;
                Response::Lookup {
                    location,
                }
            },

            "join" => {
                check_params_len(&arr, 2)?;
                Response::Join
            }

            "getsuccessor" => {
                check_params_len(&arr, 3)?;
                let location = Location::from_string(arr[2].to_string())?;
                Response::GetSuccessor {
                    location,
                }
            },

            "getpredecessor" => {
                check_params_len(&arr, 3)?;
                let location = Location::from_string(arr[2].to_string())?;
                Response::GetPredecessor {
                    location,
                }
            }

            "closestprecedingfinger" => {
                check_params_len(&arr, 3)?;
                let location = Location::from_string(arr[2].to_string())?;
                Response::ClosestPrecedingFinger {
                    location,
                }
            },

            _ => {
                return Err(
                    "Invalid response. Unrecognized response type."
                    .into());
            }
        };
        Ok(response)
    }

    pub fn serialize(&self) -> Result<String> {
        let res = match self {
            Response::Lookup { location } => {
                format!("RES LOOKUP {}", location.to_string())
            },
            Response::Join => {
                format!("RES JOIN")
            },
            Response::GetSuccessor { location } => {
                format!("RES GETSUCCESSOR {}", location.to_string())
            },
            Response::GetPredecessor{ location } => {
                format!("RES GETPREDECESSOR {}", location.to_string())
            },
            Response::ClosestPrecedingFinger { location } => {
                format!("RES CLOSESTPRECEDINGFINGER {}", location.to_string())
            }
            _ => {
                return Err("Error serializing response. Invalid response type.".into());
            }
        };
        Ok(res)
    }
}

/*
 * Assert the given string array is with given length. Otherwise throw an error.
 */
fn check_params_len(arr: &Vec<&str>, len: usize) -> Result<()> {
    if arr.len() != len {
        return Err(
            format!("Invalid command. {} command takes {} parameters.", arr[0], len)
            .into()
        );
    }
    Ok(())
}

/*
 * Convenience function to parse virtual node id.
 * Need to convert str to u8, and then make sure it is less than node_list length.
 */
fn parse_virtual_node_id(input: &str, node_list: Arc<NodeList>) -> Result<u8> {
    let virtual_node_id = str::parse::<u8>(input)?;
    if virtual_node_id as usize >= node_list.node_list.len() {
        return Err(
            "Invalid command. Virtual node number too large."
            .into());
    }
    Ok(virtual_node_id)
}

/*
 * Convenience function to parse key(id) as a big uint.
 */
fn parse_key(input: &str) -> Result<BigUint> {
    let key = match BigUint::parse_bytes(input.as_bytes(), 16) {
        Some(key) => { key },
        None => {
            return Err(
                "Invalid command. Failed to parse identifier."
                .into()
            );
        }
    };
    Ok(key)
}

async fn execute_request(request: Request, node_list: Arc<NodeList>) -> Result<Response> {
    let response = match request {
        Request::Lookup { virtual_node_id, key } => {
            let own_location = {
                let node = node_list.node_list[virtual_node_id as usize].lock().await;
                node.location.clone()
            };

            let location = process::find_successor(&own_location, &key).await?;
            Response::Lookup {
                location
            }
        },

        Request::Join { virtual_node_id, location } => {
            membership::join(node_list, virtual_node_id, location).await?;
            Response::Join
        }

        Request::GetSuccessor { virtual_node_id } => {
            let location = {
                let node = node_list.node_list[virtual_node_id as usize].lock().await;
                Location::option_to_result(&node.successor)?
            };
            Response::GetSuccessor {
                location,
            }
        },

        Request::GetPredecessor { virtual_node_id } => {
            let location = {
                let node = node_list.node_list[virtual_node_id as usize].lock().await;
                Location::option_to_result(&node.predecessor)?
            };
            Response::GetPredecessor {
                location,
            }
        }

        Request::ClosestPrecedingFinger { virtual_node_id, key } => {
            let location = {
                let node = node_list.node_list[virtual_node_id as usize].lock().await;
                node.closest_preceding_finger(key)?
            };
            Response::ClosestPrecedingFinger {
                location,
            }
        },
    };

    Ok(response)
}

/*
 * Given network I/O buffer, parse the request, and execute it.
 */
pub async fn process_request(buf: &BytesMut, node_list: Arc<NodeList>) -> Result<String> {
    /* Parse request. */
    let request = Request::parse_from_buf(buf, node_list.clone())?;
    /* Execute request. */
    let response = execute_request(request, node_list.clone()).await?;

    /* Serialize the response to be sent back to client. */
    let string = response.serialize()?;
    Ok(string)
}