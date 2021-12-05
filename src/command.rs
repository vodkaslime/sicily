use bytes::BytesMut;
use num::BigUint;
use std::sync::Arc;

use crate::location::Location;
use crate::node::NodeList;
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

/*
 * Assert the given string array is with given length. Otherwise throw an error.
 */
fn check_params_equal_to(arr: &Vec<&str>, len: usize) -> Result<()> {
    if arr.len() != len {
        return Err(
            format!("Invalid command. {} command takes {} parameters.", arr[0], len)
            .into()
        );
    }
    Ok(())
}

/*
 * Parse request from buffer.
 */
fn parse_request(buf: &BytesMut, node_list: Arc<NodeList>) -> Result<Request> {
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
            check_params_equal_to(&arr, 3)?;

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
            check_params_equal_to(&arr, 3)?;

            let virtual_node_id = str::parse::<u8>(arr[1])?;
            Request::Join {
                virtual_node_id,
                location: Location::from_string(arr[2].to_string())?,
            }
        },

        "getsuccessor" => {
            check_params_equal_to(&arr, 2)?;

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

async fn execute_request(request: Request, node_list: Arc<NodeList>) -> Result<Response> {
    let response = match request {
        Request::Lookup { virtual_node_id, key } => {
            let node = node_list.node_list[virtual_node_id as usize].lock().await;
            Response::Invalid
            
        },
        Request::GetSuccessor { virtual_node_id } => {
            let response = {
                let node = node_list.node_list[virtual_node_id as usize].lock().await;
                log::info!("The successor is: {:?}", node.successor);
                Response::GetSuccessor {
                    location: node.successor.clone(),
                }
            };
            response

        }
        _ => {
            Response::Invalid
        }
    };
    Ok(response)
}

fn serialize_response(response: Response) -> String {
    match response {
        Response::GetSuccessor{ location } => {
            format!("RES GETSUCCESSOR {}", location.to_string())
        }
        _ => {
            "Invalid".to_string()
        }
    }
}

/*
 * Given network I/O buffer, parse the request, and execute it.
 */
pub async fn process_request(buf: &BytesMut, node_list: Arc<NodeList>) -> Result<String> {
    /* Parse request. */
    let request = parse_request(buf, node_list.clone())?;

    /* Execute request. */
    let response = execute_request(request, node_list.clone()).await?;

    /* Serialize the response to be sent back to client. */
    let string = serialize_response(response);
    log::info!("The res string is: {}", string);
    Ok(string)
}