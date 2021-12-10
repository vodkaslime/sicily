use bytes::BytesMut;
use num::BigUint;
use std::sync::Arc;

use crate::location::Location;
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
    }
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

                let virtual_node_id = str::parse::<u8>(arr[1])?;
                if virtual_node_id as usize >= node_list.node_list.len() {
                    return Err(
                        "Invalid command. Virtual node number too large."
                        .into());
                }

                let key;
                match BigUint::parse_bytes(arr[2].as_bytes(), 16) {
                    Some(k) => {
                        key = k;
                    },
                    None => {
                        return Err(
                            "Invalid command. Failed to parse identifier."
                            .into()
                        );
                    }
                };

                Request::Lookup {
                    virtual_node_id,
                    key
                }
            },

            "join" => {
                check_params_len(&arr, 3)?;

                let virtual_node_id = str::parse::<u8>(arr[1])?;
                Request::Join {
                    virtual_node_id,
                    location: Location::from_string(arr[2].to_string())?,
                }
            },

            "getsuccessor" => {
                check_params_len(&arr, 2)?;

                let virtual_node_id = str::parse::<u8>(arr[1])?;
                Request::GetSuccessor {
                    virtual_node_id,
                }
            }
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
            Request::GetSuccessor { virtual_node_id } => {
                format!("GETSUCCESSOR {}", virtual_node_id)
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
    Join {
        location: Location,
    },
    GetSuccessor {
        location: Location,
    },
    Invalid,
}

impl Response {
    /*
     * Parse request from buffer.
     * Param node_list is needed here to guard virtual_node_id.
     */
    pub fn parse_from_buf(buf: &BytesMut) -> Result<Self> {
        let mut s = String::from_utf8(buf.to_vec())?;
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

        /* Start parsing request. */
        let response = match arr[1].to_lowercase().as_str() {
            "getsuccessor" => {
                check_params_len(&arr, 3)?;
                let location = Location::from_string(arr[2].to_string())?;
                Response::GetSuccessor {
                    location,
                }
            }
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
            }
            Response::GetSuccessor { location } => {
                format!("RES GETSUCCESSOR {}", location.to_string())
            },
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
        Request::GetSuccessor { virtual_node_id } => {
            let location = {
                let node = node_list.node_list[virtual_node_id as usize].lock().await;
                node.successor.clone()
            };
            Response::GetSuccessor {
                location,
            }
        }
        _ => {
            Response::Invalid
        }
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