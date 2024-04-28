use std::collections::HashMap;
use std::hash::{DefaultHasher, Hash, Hasher};
use std::io::{BufRead, BufReader, BufWriter};
use std::net::{IpAddr, TcpStream};
use std::sync::{Arc, Mutex};
use log::info;
use crate::server::{control_plane, requests};
use crate::server::cache::Key;
use crate::server::commands::ClusterState;

pub type NodeId = String;
pub type BucketId = u64;

pub struct Cluster {
    self_node_id: NodeId,
    num_buckets: u64,
    pub bucket_node_assignments: Arc<Mutex<HashMap<BucketId, NodeId>>>,
    local_buckets_keys: Arc<Mutex<HashMap<BucketId, Vec<Key>>>>,
    node_connections: Arc<Mutex<HashMap<NodeId, TcpStream>>>,
}


impl Cluster {
    pub fn new(num_buckets: u64, self_node_id: NodeId, main_node_ip: Option<IpAddr>) -> Cluster {
        let bucket_node_assignments = Arc::new(Mutex::new(HashMap::new()));
        let local_buckets_keys = Arc::new(Mutex::new(HashMap::new()));
        let node_connections = Arc::new(Mutex::new(HashMap::new()));
        match main_node_ip {
            None => {
                Self::init_self_bucket_nodes(&self_node_id, num_buckets, bucket_node_assignments.clone());

                Cluster {
                    self_node_id,
                    num_buckets,
                    bucket_node_assignments,
                    local_buckets_keys,
                    node_connections,
                }
            }
            Some(main_node) => {
                let stream = TcpStream::connect(main_node.to_string()).expect("Failed to connect to server");
                let stream_clone = stream.try_clone().unwrap();
                let mut reader = BufReader::new(stream);
                // let mut writer = BufWriter::new(stream_clone);
                let mut s = String::new();
                // on init, new node first gets cluster state (ips of all existing nodes)
                // let cluster_state: ClusterState = get_cluster_state(stream.try_clone());
                // Self::init_bucket_nodes(&self_node_id, cluster_state.nodes_to_buckets, bucket_node_assignments.clone());

                // opens connections to all the existing nodes
                // sends JoinCluster to one of the nodes (node_main)
                // node_main assigns buckets to new node
                // and sends UpdateClusterState request to all rest of nodes (to set new node responsible for those buckets)
                // let mut cluster = cluster.lock().unwrap();
                // let response = control_plane::process_client_request(request, &mut cache, &mut cluster);
                // let mut response_str = response.serialize();
                // response_str.push('\n');
                //
                // writer.write_all(response_str.as_bytes()).unwrap();
                // writer.flush().unwrap();

                Cluster {
                    self_node_id,
                    num_buckets,
                    bucket_node_assignments,
                    local_buckets_keys,
                    node_connections,
                }
            }
        }
    }

    pub fn is_key_local(&self, key: &Key) -> bool {
        self.get_node_for_key(key) == self.self_node_id
    }

    pub fn get_node_for_key(&self, key: &Key) -> NodeId {
        let bucket = self.get_bucket_for_key(key);
        self.bucket_node_assignments.lock().unwrap().get(&bucket).unwrap().clone()
    }

    pub fn add_node_connection(&mut self, node_id: NodeId, connection: TcpStream) {
        self.node_connections.lock().unwrap().insert(node_id, connection);
    }

    pub fn get_all_keys_for_bucket(&self, bucket_id: BucketId) -> Vec<Key> {
        let local_buckets = self.local_buckets_keys.lock().unwrap();
        local_buckets.get(&bucket_id).unwrap().clone()
    }

    pub fn get_connected_nodes_ips(&self) -> HashMap<NodeId, IpAddr> {
        self.node_connections.lock().unwrap().iter().map(|(node_id, stream)| {
            let ip_addr = stream.peer_addr().ok().unwrap().ip();
            (node_id.to_string(), ip_addr)
        }).collect()
    }

    fn get_bucket_for_key(&self, key: &Key) -> BucketId {
        calculate_hash(key) % self.num_buckets
    }

    fn init_self_bucket_nodes(self_id: &NodeId, num_buckets: u64, bucket_nodes: Arc<Mutex<HashMap<BucketId, NodeId>>>) {
        let mut buckets = bucket_nodes.lock().unwrap();
        for bucket_id in 0..num_buckets {
            buckets.insert(bucket_id, self_id.clone());
        }
    }

    fn init_bucket_nodes(self_id: &NodeId, cluster_nodes: HashMap<BucketId, NodeId>, bucket_nodes: Arc<Mutex<HashMap<BucketId, NodeId>>>) {
        let mut buckets = bucket_nodes.lock().unwrap();
        // for bucket_id in 0..num_buckets {
        //     buckets.insert(bucket_id, self_id.clone());
        // }
        todo!()
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