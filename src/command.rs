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

use bytes::BytesMut;
use num::BigUint;
use std::sync::Arc;

use crate::config::Config;
use crate::location::Location;
use crate::membership;
use crate::node::NodeList;
use crate::process;
use crate::utils::Result;

#[derive(Debug)]
pub enum Request {
    ClosestPrecedingFinger {
        virtual_node_id: u8,
        key: BigUint,
    },
    GetPredecessor {
        virtual_node_id: u8,
    },
    GetSuccessor {
        virtual_node_id: u8,
    },
    Info {
        virtual_node_id: u8,
    },
    Join {
        virtual_node_id: u8,
        location: Location,
    },
    Lookup {
        virtual_node_id: u8,
        key: BigUint,
    },
    Notify {
        virtual_node_id: u8,
        notifier: Location,
    }
}

impl Request {

    /*
     * Parse request from buffer.
     * Return parsed request, and whether this request is initiated by a human client,
     * which ends with "\r\n".
     * 
     * Param node_list is needed here to guard virtual_node_id.
     */
    pub fn parse_from_buf(
        buf: &BytesMut,
        node_list: Arc<NodeList>,
        config: Arc<Config>,
    ) -> Result<(Self, bool)> {
        let mut s = String::from_utf8(buf.to_vec())?;

        let mut is_human_client = false;
        /* Remove trailing "\r\n" if the command if input by user. */
        if s.ends_with("\r\n") {
            is_human_client = true;
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
            "closestprecedingfinger" => {
                check_params_len(&arr, 3)?;
                let virtual_node_id = parse_virtual_node_id(arr[1], node_list.clone())?;
                let key = parse_key(arr[2])?;
                Request::ClosestPrecedingFinger {
                    virtual_node_id,
                    key,
                }
            },
            "getpredecessor" => {
                check_params_len(&arr, 2)?;
                let virtual_node_id = parse_virtual_node_id(arr[1], node_list.clone())?;
                Request::GetPredecessor {
                    virtual_node_id,
                }
            },
            "getsuccessor" => {
                check_params_len(&arr, 2)?;
                let virtual_node_id = parse_virtual_node_id(arr[1], node_list.clone())?;
                Request::GetSuccessor {
                    virtual_node_id,
                }
            },
            "info" => {
                check_params_len(&arr, 2)?;
                let virtual_node_id = parse_virtual_node_id(arr[1], node_list.clone())?;
                Request::Info {
                    virtual_node_id,
                }
            },
            "join" => {
                check_params_len(&arr, 3)?;
                let virtual_node_id = parse_virtual_node_id(arr[1], node_list.clone())?;
                let location = Location::from_string(arr[2].to_string(), config)?;
                Request::Join {
                    virtual_node_id,
                    location,
                }
            },
            "lookup" => {
                check_params_len(&arr, 3)?;
                let virtual_node_id = parse_virtual_node_id(arr[1], node_list.clone())?;
                let key = parse_key(arr[2])?;
                Request::Lookup {
                    virtual_node_id,
                    key,
                }
            },
            "notify" => {
                check_params_len(&arr, 3)?;
                let virtual_node_id = parse_virtual_node_id(arr[1], node_list.clone())?;
                let notifier = Location::from_string(arr[2].to_string(), config)?;
                Request::Notify {
                    virtual_node_id,
                    notifier,
                }
            },
            _ => {
                return Err(
                    "Invalid command. Unrecognized command."
                    .into());
            }
        };
        Ok((command, is_human_client))
    }

    pub fn serialize(&self) -> Result<String> {
        let res = match self {
            Request::ClosestPrecedingFinger { virtual_node_id, key } => {
                format!("CLOSESTPRECEDINGFINGER {} {}", virtual_node_id, key)
            },
            Request::GetPredecessor { virtual_node_id } => {
                format!("GETPREDECESSOR {}", virtual_node_id)
            },
            Request::GetSuccessor { virtual_node_id } => {
                format!("GETSUCCESSOR {}", virtual_node_id)
            },
            Request::Info { virtual_node_id } => {
                format!("INFO {}", virtual_node_id)
            },
            Request::Join { virtual_node_id, location } => {
                format!("JOIN {} {}", virtual_node_id, location.to_string())
            },
            Request::Lookup { virtual_node_id, key } => {
                format!("LOOKUP {} {}", virtual_node_id, key)
            },
            Request::Notify { virtual_node_id, notifier } => {
                format!("NOTIFY {} {}", virtual_node_id, notifier.to_string())
            },
        };
        Ok(res)
    }
}

#[derive(Debug)]
pub enum Response {
    ClosestPrecedingFinger {
        location: Location,
    },
    GetPredecessor {
        location: Option<Location>,
    },
    GetSuccessor {
        location: Location,
    },
    Info {
        info: String,
    },
    Join,
    Lookup {
        location: Location,
    },
    Notify,
}

impl Response {
    /*
     * Parse request from buffer.
     * Param node_list is needed here to guard virtual_node_id.
     */
    pub fn parse_from_buf(buf: &BytesMut, config: Arc<Config>) -> Result<Self> {
        let s = String::from_utf8(buf.to_vec())?;
        let arr: Vec<&str> = s.split(" ").collect();

        /* Should have at least two valid string in the array vector after split. */
        if arr.len() < 2 {
            return Err(
                "Invalid response. At least two separate strings required."
                .into());
        }

        if arr[0].to_lowercase() != "res" {
            return Err(
                "Invalid response. The first string is not \"RES\"."
                .into());
        } 

        /* Start parsing response. */
        let response = match arr[1].to_lowercase().as_str() {
            "closestprecedingfinger" => {
                check_params_len(&arr, 3)?;
                let location = Location::from_string(arr[2].to_string(), config)?;
                Response::ClosestPrecedingFinger {
                    location,
                }
            },
            "getpredecessor" => {
                check_params_len(&arr, 3)?;
                if arr[2].to_lowercase() == "none" {
                    Response::GetPredecessor {
                        location: None,
                    }
                } else {
                    let location = Location::from_string(arr[2].to_string(), config)?;
                    Response::GetPredecessor {
                        location: Some(location),
                    }
                }
            },
            "getsuccessor" => {
                check_params_len(&arr, 3)?;
                let location = Location::from_string(arr[2].to_string(), config)?;
                Response::GetSuccessor {
                    location,
                }
            },
            "info" => {
                /* No need to check param number. */
                let mut info = "".to_string();
                for i in 2..arr.len() {
                    info.push_str(arr[i]);
                }
                Response::Info {
                    info,
                }
            },
            "join" => {
                check_params_len(&arr, 2)?;
                Response::Join
            }
            "lookup" => {
                check_params_len(&arr, 3)?;
                let location = Location::from_string(arr[2].to_string(), config)?;
                Response::Lookup {
                    location,
                }
            },
            "notify" => {
                check_params_len(&arr, 2)?;
                Response::Notify
            },
            _ => {
                return Err(
                    "Invalid response. Unrecognized response type."
                    .into());
            }
        };
        Ok(response)
    }

    pub fn serialize(&self, is_human_client: bool) -> Result<String> {
        let mut res = match self {
            Response::ClosestPrecedingFinger { location } => {
                format!("RES CLOSESTPRECEDINGFINGER {}", location.to_string())
            },
            Response::GetPredecessor{ location } => {
                match location {
                    Some(location) => format!("RES GETPREDECESSOR {}", location.to_string()),
                    None => format!("RES GETPREDECESSOR NONE"),
                }
            },
            Response::GetSuccessor { location } => {
                format!("RES GETSUCCESSOR {}", location.to_string())
            },
            Response::Info { info } => {
                format!("RES INFO {}", info)
            }
            Response::Join => {
                format!("RES JOIN")
            },
            Response::Lookup { location } => {
                format!("RES LOOKUP {}", location.to_string())
            },
            Response::Notify => {
                format!("RES NOTIFY")
            },
        };
        if is_human_client {
            res.push_str("\r\n");
        }
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
    let key = match BigUint::parse_bytes(input.as_bytes(), 10) {
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

async fn execute_request(
    request: Request,
    node_list: Arc<NodeList>,
    config: Arc<Config>,
) -> Result<Response> {
    let response = match request {
        Request::ClosestPrecedingFinger { virtual_node_id, key } => {
            let location = {
                let node = node_list.node_list[virtual_node_id as usize].lock().await;
                node.closest_preceding_finger(key)?
            };
            Response::ClosestPrecedingFinger {
                location,
            }
        },
        Request::GetPredecessor { virtual_node_id } => {
            let location = {
                let node = node_list.node_list[virtual_node_id as usize].lock().await;
                match node.get_predecessor() {
                    Ok(location) => Some(location),
                    Err(_) => None,
                }
            };
            Response::GetPredecessor {
                location,
            }
        },
        Request::GetSuccessor { virtual_node_id } => {
            let location = {
                let node = node_list.node_list[virtual_node_id as usize].lock().await;
                node.get_successor()?
            };
            Response::GetSuccessor {
                location,
            }
        },
        Request::Info { virtual_node_id } => {
            let info = {
                let node = node_list.node_list[virtual_node_id as usize].lock().await;
                node.get_info()
            };
            Response::Info {
                info,
            }
        }
        Request::Join { virtual_node_id, location } => {
            membership::join(virtual_node_id, location, node_list, config).await?;
            Response::Join
        },
        Request::Lookup { virtual_node_id, key } => {
            let own_location = {
                let node = node_list.node_list[virtual_node_id as usize].lock().await;
                node.own_location()
            };

            let location = process::find_successor(&own_location, &key, config).await?;
            Response::Lookup {
                location
            }
        },
        Request::Notify { virtual_node_id, notifier } => {
            {
                let mut node = node_list.node_list[virtual_node_id as usize].lock().await;
                node.notify_with(&notifier);
            }
            Response::Notify
        }
    };

    Ok(response)
}

/*
 * Given network I/O buffer, parse the request, and execute it.
 */
pub async fn process_request(
    buf: &BytesMut,
    node_list: Arc<NodeList>,
    config: Arc<Config>,
) -> Result<String> {
    /* Parse request. */
    let (request, is_human_client) = Request::parse_from_buf(
        buf,
        node_list.clone(),
        config.clone()
    )?;

    /* Execute request. */
    let response = execute_request(request, node_list.clone(), config.clone()).await?;

    /* Serialize the response to be sent back to client. */
    let string = response.serialize(is_human_client)?;
    Ok(string)
}