use num::bigint::BigUint;

use crate::arithmetic;
use crate::command::{ Request, Response };
use crate::client::Client;
use crate::location::Location;
use crate::utils::Result;

pub async fn find_successor(location: &Location, id: &BigUint) -> Result<Location> {
    let pred = find_predecessor(location, id).await?;
    return get_successor(&pred).await;
}

async fn find_predecessor(location: &Location, id: &BigUint) -> Result<Location> {
    let mut location = location.clone();
    while !arithmetic::is_in_range(
        id,
        (&location.identifier, false),
        (&get_successor(&location).await?.identifier, true)
    ) {
        location = find_closest_preceding_finger(&location, &id).await?;
    }
    Ok(location)
}

async fn get_successor(location: &Location) -> Result<Location> {
    let request = Request::GetSuccessor {
        virtual_node_id: location.virtual_node_id,
    };
    let mut client = Client::new(location).await?;
    client.send_request(request).await?;
    let response = client.receive().await?;
    let res_location = match response {
        Response::GetSuccessor { location } => location,
        _ => {
            return Err(
                "Error receiving response while doing GET_SUCCESSOR. Got invalid response type."
                .into()
            );
        }
    };
    Ok(res_location)
}

async fn find_closest_preceding_finger(location: &Location, id: &BigUint) -> Result<Location> {
    Ok(location.clone())
}