# Sicily
An implementation of Chord p2p lookup protocol and service, based on paper https://pdos.csail.mit.edu/papers/chord:sigcomm01/chord_sigcomm.pdf, in rust and async I/O flavor to achieve high performance.

This implementation has virtual node internally supported. It uses SHA256 as hashing algorithm. It supports basic functionaliy of lookup, join and stabilize of the cluster. Failure detection is not supported yet.

## Getting Started

```
RUST_LOG="debug" cargo run -- --host 127.0.0.1
```

You can specify `host`, `port`, `output buffer size`, `stabilize frequency`, `id bits` and `virtual node number` when starting the service. For more details about configuration, refer to `src/config.rs`.

## Basic Functionality
To perform operations with the running Sicily service, you can use an upper level application demanding lookups (for example database or any other kind of distributed storage system), or just a telnet as a client.

Here just take telnet as an example.

### Lookup
Connect to service and do:

```
LOOKUP <virtual_node_id> <key>
```
for example:
```
LOOKUP 0 42
```

This triggers a look up, asking node at `virtual_node_id`, about the successor (owner) of `key`.

The `key` here is a decimal number, which is the mod of hashed original plain text key. For more details, refer to the paper.

If successful, you will receive a response:
```
RES LOOKUP <owner_host>:<owner_port>:<owner_virtual_node_id>
```

### Join
Connect to service and do:

```
JOIN <src_virtual_node_id> <target_host>:<target_port>:<target_virtual_node_id>
```
for example:

```
JOIN 0 127.0.0.1:8820:1
```

This triggers a join operation, asking node at `src_virtual_node_id` to join with the `target` node specified above.

If successful, you will receive a response:
```
RES JOIN
```
And the cluster will later communicate and stabilize in the async manner since we have stabilization working in the background.