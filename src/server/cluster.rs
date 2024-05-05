use std::collections::HashMap;
use std::hash::{DefaultHasher, Hash, Hasher};
use std::io::{BufRead, BufReader, BufWriter, Write};
use std::net::{SocketAddr, TcpStream};
use std::sync::{Arc, Mutex};
use log::{error, info};
use crate::server::cache::Key;
use crate::server::commands::CmdResponseEnum;
use crate::server::commands::CommandsEnum::GetClusterState;

pub type NodeId = String;
pub type BucketId = u64;

pub struct Cluster {
    self_node_id: NodeId,
    num_buckets: u64,
    bucket_node_assignments: Arc<Mutex<HashMap<BucketId, NodeId>>>,
    local_buckets_keys: Arc<Mutex<HashMap<BucketId, Vec<Key>>>>,
    node_connections: Arc<Mutex<HashMap<NodeId, TcpStream>>>,
}


impl Cluster {
    pub fn new(num_buckets: u64, self_node_id: NodeId, leader_ip: Option<SocketAddr>) -> Cluster {
        let bucket_node_assignments = Arc::new(Mutex::new(HashMap::new()));
        let local_buckets_keys = Arc::new(Mutex::new(HashMap::new()));
        let node_connections = Arc::new(Mutex::new(HashMap::new()));
        match leader_ip {
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
            Some(leader_node) => {
                Self::handle_cluster_join(&self_node_id, leader_node, bucket_node_assignments.clone(), node_connections.clone());

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

    pub fn get_bucket_node_assignments(&self) -> HashMap<BucketId, NodeId> {
        self.bucket_node_assignments.lock().unwrap().clone()
    }

    pub fn get_all_keys_for_bucket(&self, bucket_id: BucketId) -> Vec<Key> {
        let local_buckets = self.local_buckets_keys.lock().unwrap();
        local_buckets.get(&bucket_id).unwrap().clone()
    }

    pub fn get_connected_nodes_ips(&self) -> HashMap<NodeId, SocketAddr> {
        self.node_connections.lock().unwrap().iter().map(|(node_id, stream)| {
            let socket_addr = stream.peer_addr().unwrap();
            (node_id.to_string(), socket_addr)
        }).collect()
    }

    fn handle_cluster_join(self_node_id: &NodeId, leader_node: SocketAddr, bucket_node_assignments: Arc<Mutex<HashMap<BucketId, NodeId>>>, node_connections: Arc<Mutex<HashMap<NodeId, TcpStream>>>) {
        let stream = TcpStream::connect(leader_node.to_string()).expect("Failed to connect to server");
        let cluster_state = Self::request_cluster_state(stream.try_clone().unwrap());
        Self::init_bucket_nodes(&cluster_state, bucket_node_assignments.clone(), node_connections.clone());
        Self::join_cluster(self_node_id, stream.try_clone().unwrap());
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

    fn init_bucket_nodes(cluster_state: &CmdResponseEnum, _self_bucket_nodes: Arc<Mutex<HashMap<BucketId, NodeId>>>, _self_node_connections: Arc<Mutex<HashMap<NodeId, TcpStream>>>) {
        match cluster_state {
            CmdResponseEnum::ClusterState { buckets_to_nodes, nodes_to_ips } => {
                // opens connections to all the existing nodes
                // let mut buckets = bucket_nodes.lock().unwrap();
                buckets_to_nodes.iter().for_each(|(bucket, node)| {
                    info!("Bucket {bucket} is handled by {node}");
                });
                nodes_to_ips.iter().for_each(|(node_id, ip)| {
                    info!("Node {node_id} has ip: {ip}");
                });

                // for bucket_id in 0..num_buckets {
                //     buckets.insert(bucket_id, self_id.clone());
                // }
            }
            _ => error!("")
        }
    }

    fn request_cluster_state(stream: TcpStream) -> CmdResponseEnum {
        let mut reader = BufReader::new(stream.try_clone().unwrap());
        let mut writer = BufWriter::new(stream.try_clone().unwrap());
        let command = GetClusterState {};
        let mut command_str = serde_json::to_string(&command).unwrap();
        command_str.push('\n');
        writer.write_all(command_str.as_bytes()).unwrap();
        writer.flush().unwrap();

        let mut s = String::new();
        reader.read_line(&mut s).unwrap();
        info!("Received leader response: {s}");
        let r = serde_json::from_str(&s).unwrap();
        return r;
    }

    fn join_cluster(_self_node_id: &NodeId, _stream: TcpStream) {
        // sends JoinCluster to one of the nodes (leader)
        // leader assigns buckets to new node
        // leader sends UpdateClusterState request to all rest of nodes (to set new node responsible for those buckets)
        // leader responds to new node with list of keys it now handles
        // new node may catch up on keys, but may as well ignore that
    }
}

fn calculate_hash<T: Hash>(t: &T) -> u64 {
    let mut s = DefaultHasher::new();
    t.hash(&mut s);
    s.finish()
}
