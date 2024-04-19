# rusty-cache
Distributed cache in Rust

This is a learning project, and it will never become anything production ready.
It's almost first Rust code I'm writing in my life.

### Planned:
 - distributed cache, 
 - multiple servers
 - no replication
 - each can accept values and pass them to correct node.

### What should be implemented:
 - setting TTL on set
 - background process to evict keys after TTL
 - additional data types
 - accepting data from TCP messages

### Components
 - server
   - accepts list of other servers on start
   - TODO: how to set up key distribution between nodes
 - client
   - connects to specified server on start, rest of functionality same as local tester

### What can be added further
 - monitoring
 - replication
 - additional data types

