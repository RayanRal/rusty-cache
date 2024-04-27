use std::collections::HashMap;
use std::hash::{DefaultHasher, Hash, Hasher};
use std::net::TcpStream;
use std::sync::{Arc, Mutex};


pub struct Cluster {
    self_node_id: String,
    num_buckets: u64,
    // bucket_id -> node_id
    bucket_node_assignments: Arc<Mutex<HashMap<String, String>>>,
    //
    local_buckets_keys: Arc<Mutex<HashMap<String, Vec<String>>>>,
    // node_id -> connection
    node_connections: Arc<Mutex<HashMap<String, TcpStream>>>,
}


impl Cluster {
    pub fn new(num_buckets: u64, self_node_id: String) -> Cluster {
        let bucket_node_assignments = Arc::new(Mutex::new(HashMap::new()));
        let local_buckets_keys = Arc::new(Mutex::new(HashMap::new()));
        let node_connections = Arc::new(Mutex::new(HashMap::new()));

        Self::init_buckets(&self_node_id, num_buckets, bucket_node_assignments.clone());

        Cluster {
            self_node_id,
            num_buckets,
            bucket_node_assignments,
            local_buckets_keys,
            node_connections,
        }
    }

    pub fn is_key_local(&self, key: &String) -> bool {
        self.get_node_for_key(key) == self.self_node_id
    }

    pub fn get_node_for_key(&self, key: &String) -> String {
        let bucket = self.get_bucket_for_key(key).to_string();
        self.bucket_node_assignments.lock().unwrap().get(&bucket).unwrap().clone()
    }

    pub fn add_node(&mut self, server_id: String, connection: TcpStream) {
        self.node_connections.lock().unwrap().insert(server_id, connection);
    }

    pub fn get_all_keys_for_bucket(&self, bucket_id: String) -> Vec<String> {
        let local_buckets = self.local_buckets_keys.lock().unwrap();
        local_buckets.get(&bucket_id).unwrap().clone()
    }
    
    pub fn get_node_ids(&self) -> Vec<String> {
        self.node_connections.lock().unwrap().keys().cloned().collect()
    }

    fn get_bucket_for_key(&self, key: &String) -> u64 {
        calculate_hash(key) % self.num_buckets
    }

    fn init_buckets(self_id: &String, num_buckets: u64, bucket_servers: Arc<Mutex<HashMap<String, String>>>) {
        let mut buckets = bucket_servers.lock().unwrap();
        for bucket_id in 0..num_buckets {
            buckets.insert(bucket_id.to_string(), self_id.clone());
        }
    }
}

fn calculate_hash<T: Hash>(t: &T) -> u64 {
    let mut s = DefaultHasher::new();
    t.hash(&mut s);
    s.finish()
}

// - server has hashmap bucket_id (16) -> list of keys
// - server 1 has a map bucket -> server
// - server 2 comes up, connects to server 1, sends `get_buckets_to_handle` request
// - server 1 updates bucket -> server map, assigns buckets to server 2