use std::collections::HashMap;
use std::hash::{DefaultHasher, Hash, Hasher};
use std::net::TcpStream;
use std::sync::{Arc, Mutex};


pub struct Cluster {
    self_id: String,
    num_buckets: u64,
    
    // bucket_id -> server_ip
    bucket_servers: Arc<Mutex<HashMap<String, String>>>,
    // server_ip -> connection
    server_connections: Arc<Mutex<HashMap<String, TcpStream>>>,
}


impl Cluster {
    pub fn new(num_buckets: u64, self_id: String) -> Cluster {
        let bucket_servers: Arc<Mutex<HashMap<String, String>>> = Arc::new(Mutex::new(HashMap::new()));
        let server_connections: Arc<Mutex<HashMap<String, TcpStream>>> = Arc::new(Mutex::new(HashMap::new()));
        
        // todo: init all bucket servers with server 1

        Cluster {
            self_id,
            num_buckets,
            bucket_servers,
            server_connections,
        }
    }

    pub fn get_server_for_key(&mut self, key: &String) -> String {
        let bucket = self.get_bucket_for_key(key).to_string();
        self.bucket_servers.lock().unwrap().get(&bucket).unwrap().clone()
    }
    
    pub fn add_server(&mut self, server_id: String, connection: TcpStream) {
        
    }

    fn get_bucket_for_key(&mut self, key: &String) -> u64 {
        calculate_hash(key) % self.num_buckets
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