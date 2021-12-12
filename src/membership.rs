use std::sync::Arc;
use rand::prelude::*;

use crate::arithmetic;
use crate::client::Client;
use crate::command::{ Request, Response };
use crate::location::Location;
use crate::node::NodeList;
use crate::process;
use crate::utils::Result;

/*
 * Function called every time a new node tries to join in a cluster
 * by talking to location, which represents a node that is already in the cluster.
 */
pub async fn join(
    virtual_node_id: u8,
    location: Location,
    node_list: Arc<NodeList>,
) -> Result<()> {

    /* 1. Retrieve the local identifier of the node. */
    let key = {
        let node = node_list.node_list[virtual_node_id as usize].lock().await;
        node.location.identifier.clone()
    };

    /* 2. Based on the identifier, retrieve the successor. */
    let successor = process::find_successor(&location, &key).await?;

    /* 3. Update the node's metadata. */
    {
        let mut node = node_list.node_list[virtual_node_id as usize].lock().await;
        node.predecessor = None;
        node.successor = Some(successor);
    }
    Ok(())
}

/*
 * Periodic function called to stabilize the metadata of nodes in the cluster.
 */
pub async fn stablize(virtual_node_id: u8, node_list: Arc<NodeList>) -> Result<()> {
    let (mut successor, local_location) = {
        let node = node_list.node_list[virtual_node_id as usize].lock().await;
        let successor = Location::option_to_result(&node.successor)?;
        (successor, node.location.clone())
    };

    let predecessor_of_successor = process::get_predecessor(&successor).await?;
    if arithmetic::is_in_range(
        &predecessor_of_successor.identifier,
        (&local_location.identifier, false),
        (&successor.identifier, false)) {
            {
                let mut node = node_list.node_list[virtual_node_id as usize].lock().await;
                node.successor = Some(predecessor_of_successor.clone());
            }
            successor = predecessor_of_successor.clone();
        }
    
    notify(local_location, successor).await?;
    Ok(())
}

/*
 * Notify a supposedly successor (target_location) of a given node (local_location).
 * The successor, after receiving the notification, will make a decision whether
 * it needs to update its predecessor pointer to the local_location.
 */
async fn notify(local_location: Location, target_location: Location) -> Result<()> {
    let request = Request::Notify {
        virtual_node_id: target_location.virtual_node_id,
        notifier: local_location,
    };
    let mut client = Client::new(&target_location).await?;
    client.send_request(request).await?;
    let response = client.receive().await?;
    match response {
        Response::Notify => {
            /* Happy case, nothing to do. */
        },
        _ => {
            return Err(
                "Error receiving response while doing NOTIFY. Got unexpected response type."
                .into()
            );
        }
    }
    Ok(())
}

/*
 * Periodic function to randomly pick a finger and fix it by contacting with the cluster.
 */
pub async fn fix_fingers(virtual_node_id: u8, node_list: Arc<NodeList>) -> Result<()> {

    /* 1. Pick a random finger to fix. */
    let (index, start_identifier, local_location) = {
        let mut rng = rand::rngs::StdRng::from_entropy();
        let node = node_list.node_list[virtual_node_id as usize].lock().await;
        let index = rng.gen_range(1..node.finger.len());
        let start_identifier = node.finger_start_identifier[index].clone();
        let local_location = node.location.clone();
        (index, start_identifier, local_location)
    };

    /* 2. Communicate with the cluster. */
    let successor = process::find_successor(&local_location, &start_identifier).await?;

    /* 3. Update the finger. */
    {
        let mut node = node_list.node_list[virtual_node_id as usize].lock().await;
        node.finger[index] = Some(successor);
    }
    Ok(())
}