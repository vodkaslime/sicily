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

use std::sync::Arc;
use rand::prelude::*;

use crate::arithmetic;
use crate::client::Client;
use crate::command::{ Request, Response };
use crate::config::Config;
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
    config: Arc<Config>,
) -> Result<()> {

    /* 1. Retrieve the local identifier of the node. */
    let key = {
        let node = node_list.node_list[virtual_node_id as usize].lock().await;
        node.own_location().identifier
    };

    /* 2. Based on the identifier, retrieve the successor. */
    let successor = process::find_successor(&location, &key, config).await?;

    /* 3. Update the node's metadata. */
    {
        let mut node = node_list.node_list[virtual_node_id as usize].lock().await;
        node.set_predecessor(None);
        node.set_successor(Some(successor));
    }
    Ok(())
}

/*
 * Periodic function called to stabilize the metadata of nodes in the cluster.
 */
pub async fn stablize(
    virtual_node_id: u8,
    node_list: Arc<NodeList>,
    config: Arc<Config>,
) -> Result<()> {
    let (mut successor, local_location) = {
        let node = node_list.node_list[virtual_node_id as usize].lock().await;
        let successor = node.get_successor()?;
        (successor, node.own_location())
    };

    let option = process::get_predecessor(&successor, config.clone()).await?;
    let predecessor_of_successor = match option {
        Some(location) => location,
        None => {
            /* It's OK to have a None response here when trying to get a successor node's predecessor.
             * It can be due to:
             * - The successor node is initially already in a cluster.
             * - Its predecessor (the caller node of this function) keeps getting doing the get_predecessor()
             *   to it.
             * - At the very moment, the successor node tries to join some other node, maybe in another cluster.
             * - Thus the predecessor field of the successor node is marked as None.
             * - Therefore we get into this scenario. */
            return Ok(())
        }
    };

    if arithmetic::is_in_range(
        &predecessor_of_successor.identifier,
        (&local_location.identifier, false),
        (&successor.identifier, false)) {
            {
                let mut node = node_list.node_list[virtual_node_id as usize].lock().await;
                node.set_successor(Some(predecessor_of_successor.clone()));
            }
            successor = predecessor_of_successor.clone();
        }
    
    notify(
        local_location,
        successor,
        config,
    ).await?;
    Ok(())
}

/*
 * Notify a supposedly successor (target_location) of a given node (local_location).
 * The successor, after receiving the notification, will make a decision whether
 * it needs to update its predecessor pointer to the local_location.
 */
async fn notify(
    local_location: Location,
    target_location: Location,
    config: Arc<Config>,
) -> Result<()> {
    let request = Request::Notify {
        virtual_node_id: target_location.virtual_node_id,
        notifier: local_location,
    };
    let mut client = Client::new(&target_location).await?;
    client.send_request(request).await?;
    let response = client.receive(config).await?;
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
pub async fn fix_fingers(
    virtual_node_id: u8,
    node_list: Arc<NodeList>,
    config: Arc<Config>,
) -> Result<()> {

    /* 1. Pick a random finger to fix. */
    let (index, start_identifier, local_location) = {
        let mut rng = rand::rngs::StdRng::from_entropy();
        let node = node_list.node_list[virtual_node_id as usize].lock().await;
        let index = rng.gen_range(1..node.get_finger_len());
        let start_identifier = node.get_finger_start_identifier(index)?;
        let local_location = node.own_location();
        (index, start_identifier, local_location)
    };

    /* 2. Communicate with the cluster. */
    let successor = process::find_successor(&local_location, &start_identifier, config).await?;

    /* 3. Update the finger. */
    {
        let mut node = node_list.node_list[virtual_node_id as usize].lock().await;
        node.set_finger(index, Some(successor))?;
    }
    Ok(())
}