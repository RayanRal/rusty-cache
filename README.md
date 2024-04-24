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

### What can be added further

- monitoring
- replication
- additional data types

