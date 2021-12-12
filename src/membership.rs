use crate::location::Location;
use crate::node::NodeList;
use crate::process;
use crate::utils::Result;

pub async fn join(
    node_list: NodeList,
    virtual_node_id: u8,
    location: Location) -> Result<()> {

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