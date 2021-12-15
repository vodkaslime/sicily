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

use crate::arithmetic;
use crate::command::{ Request, Response };
use crate::config::Config;
use crate::client::Client;
use crate::location::Location;
use crate::utils::Result;

/*
 * Find successor node of a key, starting by asking node at location.
 */
pub async fn find_successor(
    location: &Location,
    key: &BigUint,
    config: Arc<Config>,
) -> Result<Location> {
    let pred = find_predecessor(location, key, config.clone()).await?;
    return get_successor(&pred, config).await;
}

/*
 * Find predecessor node of a key, starting by asking node at location.
 * This function is a part of lookup process.
 */
async fn find_predecessor(
    location: &Location,
    key: &BigUint,
    config: Arc<Config>
) -> Result<Location> {
    let mut location = location.clone();
    while !arithmetic::is_in_range(
        key,
        (&location.identifier, false),
        (&get_successor(&location, config.clone()).await?.identifier, true)
    ) {
        location = find_closest_preceding_finger(&location, &key, config.clone()).await?;
    }
    Ok(location)
}

/*
 * Find successor node of a node at location.
 */
async fn get_successor(location: &Location, config: Arc<Config>) -> Result<Location> {
    let request = Request::GetSuccessor {
        virtual_node_id: location.virtual_node_id,
    };
    let mut client = Client::new(location).await?;
    client.send_request(request).await?;
    let response = client.receive(config).await?;
    let res_location = match response {
        Response::GetSuccessor { location } => location,
        _ => {
            return Err(
                "Error receiving response while doing GETSUCCESSOR. Got unexpected response type."
                .into()
            );
        }
    };
    Ok(res_location)
}

/*
 * Find predecessor node of a node at location.
 */
pub async fn get_predecessor(location: &Location, config: Arc<Config>) -> Result<Option<Location>> {
    let request = Request::GetPredecessor {
        virtual_node_id: location.virtual_node_id,
    };
    let mut client = Client::new(location).await?;
    client.send_request(request).await?;
    let response = client.receive(config).await?;
    let res_location = match response {
        Response::GetPredecessor { location } => location,
        _ => {
            return Err(
                "Error receiving response while doing GETPREDECESSOR. Got unexpected response type."
                .into()
            );
        }
    };
    Ok(res_location)
}

/*
 * Find closest preceding node of a key, by finding from fingers of a node at location.
 */
async fn find_closest_preceding_finger(
    location: &Location,
    key: &BigUint,
    config: Arc<Config>,
) -> Result<Location> {
    let request = Request::ClosestPrecedingFinger {
        virtual_node_id: location.virtual_node_id,
        key: key.clone(),
    };
    let mut client = Client::new(location).await?;
    client.send_request(request).await?;
    let response = client.receive(config).await?;
    let res_location = match response {
        Response::ClosestPrecedingFinger { location } => location,
        _ => {
            return Err(
                "Error receiving response while doing CLOSESTPRECEDINGFINGER. Got unexpected response type."
                .into()
            );
        }
    };
    Ok(res_location)
}