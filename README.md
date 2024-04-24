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

### Details - server

- server 1 comes up, it handles all requests
- it has open server channel
- server has hashmap bucket_id (16) -> list of keys
- server 1 has a map bucket -> server
- server 2 comes up, connects to server 1, sends `get_buckets_to_handle` request
- server 1 updates bucket -> server map, assigns buckets to server 2
- server 1 calls `put` with each key for server 2
- when client sends `get` - server 1 checks bucket -> list of keys and bucket -> server tables, and replies with `move`

### What can be added further

- monitoring
- replication
- additional data types
- bloom filters for key existence check

