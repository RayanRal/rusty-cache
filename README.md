# rusty-cache

Distributed cache in Rust

This is a learning project, and it will never become anything production ready.
It's almost first Rust code I'm writing in my life.

### Planned:

- distributed cache
- multiple servers
- no replication
- each can accept values and pass them to correct node

### What should be implemented:

- [x] accepting data from TCP messages
- [x] setting TTL on set
- [x] background process to evict keys after TTL
- [ ] additional data types

### How would functionality be distributed

- server
    - v1: accepts lists of other servers on start
    - knows which server is responsible for which key
    - can answer to client with redirect response
- client
    - gets a list of servers on the start
    - keeps some table of server <> key mapping (?)

### Details - server addition

- server 1 comes up, 1 server in cluster, handles all requests
  - it has server channel and client channel
  - server has hashmap bucket_id (16) -> list of keys
- server has a map bucket_id -> server ip
- server 2 comes up, connects to server 1, sends `join_cluster` request
- server 1 updates bucket_id -> server map, assigns buckets to server 2
  - need to update other servers with fresh `cluster_state`
- server 1 calls `bulk_put` with each key for server 2

### Details - client-server interaction
- client can join any server in the cluster
- when client sends `get` request, server
  - checks if it can serve the request
  - if it can't, it acts as a proxy, sending request to server 2, and returning response
- when client sends `put`
  - server checks if it can save the key
  - if not, it proxies put to another server
- this makes client totally oblivious to state of cluster and interactions with it

### What can be added further

- monitoring
- replication
- additional data types
- bloom filters for key existence check

