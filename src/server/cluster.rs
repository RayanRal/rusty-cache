use std::collections::HashMap;
use std::hash::{DefaultHasher, Hash, Hasher};
use std::io::{BufRead, BufReader, BufWriter, Write};
use std::net::{SocketAddr, TcpStream};
use std::sync::{Arc, Mutex};
use log::{error, info};
use crate::server::cache::Key;
use crate::server::commands::CmdResponseEnum;
use crate::server::commands::CommandsEnum::{GetClusterState, JoinCluster};

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

    pub fn redistribute_buckets(&self) {
        let mut nodes_to_buckets: HashMap<NodeId, Vec<BucketId>> = HashMap::new();
        for (bucket_id, node_id) in self.bucket_node_assignments.lock().unwrap().iter() {
            nodes_to_buckets.entry(node_id.to_string()).or_insert_with(Vec::new).push(*bucket_id);
        }
        let mut sorted_nodes: Vec<_> = nodes_to_buckets.into_iter().collect();
        sorted_nodes.sort_by_key(|(_, v)| v.len());
        let (most_loaded_node, nodes_buckets) = sorted_nodes.get(0).unwrap();
        let (no_buckets_node, _) =  sorted_nodes.last().unwrap();
        let buckets_to_transfer = &nodes_buckets[..(nodes_buckets.len() / 2)];
        for bucket_id in buckets_to_transfer {
            self.bucket_node_assignments.lock().unwrap().insert(*bucket_id, no_buckets_node.to_string());
        }
        // nodes_to_buckets = sorted_nodes.into_iter().collect();
        // check distribution of existing buckets to nodes
        // find nodes with too many buckets
        // take first half of those buckets
        // take all keys from those buckets
        // assign them to new node
        // TODO: eventually update 
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
        info!("Received cluster state: {s}");
        serde_json::from_str(&s).unwrap()
    }

    fn join_cluster(self_node_id: &NodeId, stream: TcpStream) {
        let mut reader = BufReader::new(stream.try_clone().unwrap());
        let mut writer = BufWriter::new(stream.try_clone().unwrap());
        let command = JoinCluster { node_id: self_node_id.to_string() };
        let mut command_str = serde_json::to_string(&command).unwrap();
        command_str.push('\n');
        writer.write_all(command_str.as_bytes()).unwrap();
        writer.flush().unwrap();

        let mut s = String::new();
        reader.read_line(&mut s).unwrap();
        info!("Received join cluster response: {s}");
        match serde_json::from_str(&s).unwrap()  {
            CmdResponseEnum::ClusterState { nodes_to_ips, buckets_to_nodes } => {
                
            }
            _ => {}
        }
        
        // TODO: leader responds to node with list of keys it now handles - save them
    }
}

fn calculate_hash<T: Hash>(t: &T) -> u64 {
    let mut s = DefaultHasher::new();
    t.hash(&mut s);
    s.finish()
}
