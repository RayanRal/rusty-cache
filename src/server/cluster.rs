use std::collections::HashMap;
use std::hash::{DefaultHasher, Hash, Hasher};
use std::io::{BufRead, BufReader, BufWriter, Write};
use std::net::{SocketAddr, TcpStream};
use std::sync::{Arc, Mutex};
use log::{error, info, warn};
use crate::server::cache::Key;
use crate::server::commands::{CmdResponseEnum, CommandsEnum};
use crate::server::commands::CommandsEnum::{GetClusterState, JoinCluster, UpdateClusterState};

pub type NodeId = String;
pub type BucketId = u64;

pub struct Cluster {
    pub self_node_id: NodeId,
    num_buckets: u64,
    bucket_node_assignments: Arc<Mutex<HashMap<BucketId, NodeId>>>,
    node_connections: Arc<Mutex<HashMap<NodeId, Arc<Mutex<TcpStream>>>>>,
}

impl Cluster {
    pub fn update_cluster_state(&self,
                                nodes_to_ips_updated: HashMap<NodeId, SocketAddr>,
                                buckets_to_nodes_updated: HashMap<BucketId, NodeId>,
    ) {
        // updating node connections
        for node in self.node_connections.lock().unwrap().keys() {
            if !nodes_to_ips_updated.contains_key(node) {
                self.node_connections.lock().unwrap().remove(node);
            }
        }
        for (node, addr) in nodes_to_ips_updated {
            self.node_connections.lock().unwrap().entry(node).or_insert_with(|| {
                Arc::new(Mutex::new(TcpStream::connect(addr).expect("Couldn't connect to new node")))
            });
        }
        // updating buckets
        for (bucket, node) in buckets_to_nodes_updated {
            self.bucket_node_assignments.lock().unwrap().insert(bucket, node);
        }
    }
}

impl Cluster {
    pub fn new(num_buckets: u64, self_node_id: NodeId, leader_ip: Option<SocketAddr>) -> Cluster {
        let bucket_node_assignments = Arc::new(Mutex::new(HashMap::new()));
        let node_connections = Arc::new(Mutex::new(HashMap::new()));

        match leader_ip {
            None => {
                Self::init_self_bucket_nodes(&self_node_id, num_buckets, bucket_node_assignments.clone());

                Cluster {
                    self_node_id,
                    num_buckets,
                    bucket_node_assignments,
                    node_connections,
                }
            }
            Some(leader_node) => {
                Self::handle_cluster_join(&self_node_id, leader_node, bucket_node_assignments.clone(), node_connections.clone());

                Cluster {
                    self_node_id,
                    num_buckets,
                    bucket_node_assignments,
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
        self.node_connections.lock().unwrap().insert(node_id, Arc::new(Mutex::new(connection)));
    }

    pub fn get_bucket_node_assignments(&self) -> HashMap<BucketId, NodeId> {
        self.bucket_node_assignments.lock().unwrap().clone()
    }

    pub fn get_cluster_node_ips(&self) -> HashMap<NodeId, SocketAddr> {
        self.node_connections.lock().unwrap().iter().map(|(node_id, arc_stream)| {
            let stream = arc_stream.lock().unwrap();
            let socket_addr = stream.peer_addr().unwrap();
            (node_id.to_string(), socket_addr)
        }).collect()
    }

    pub fn notify_cluster_nodes(&self, command: CommandsEnum) {
        for (node_id, arc_stream) in self.node_connections.lock().unwrap().iter() {
            info!("Notifying {node_id}");
            let stream = arc_stream.lock().unwrap().try_clone().unwrap();
            let mut writer = BufWriter::new(stream);
            let mut command_str = serde_json::to_string(&command).unwrap();
            command_str.push('\n');
            writer.write_all(command_str.as_bytes()).unwrap();
            writer.flush().unwrap();
        }
    }

    pub fn redistribute_buckets(&self) {
        let mut nodes: Vec<NodeId> = self.node_connections.lock().unwrap().keys().cloned().collect();
        nodes.push(self.self_node_id.to_string());
        nodes.sort();
        let mut buckets: Vec<BucketId> = self.bucket_node_assignments.lock().unwrap().keys().cloned().collect();
        buckets.sort();
        info!("redistributing nodes: {nodes:?}, buckets: {buckets:?}");
        let buckets_per_node = buckets.len() / nodes.len();
        let buckets_iter = buckets.chunks(buckets_per_node);
        self.bucket_node_assignments.lock().unwrap().clear();
        for (node_id, buckets) in nodes.iter().zip(buckets_iter) {
            for bucket in buckets {
                self.bucket_node_assignments.lock().unwrap().insert(*bucket, node_id.clone());
            }
        }
    }

    pub fn get_node_connection(&self, target_node: &NodeId) -> Option<Arc<Mutex<TcpStream>>> {
        self.node_connections.lock().unwrap()
            .get(target_node).cloned()
    }

    fn handle_cluster_join(self_node_id: &NodeId,
                           leader_node: SocketAddr,
                           bucket_node_assignments: Arc<Mutex<HashMap<BucketId, NodeId>>>,
                           node_connections: Arc<Mutex<HashMap<NodeId, Arc<Mutex<TcpStream>>>>>,
    ) {
        let stream = TcpStream::connect(leader_node.to_string()).expect("Failed to connect to server");
        let cluster_state = Self::request_cluster_state(stream.try_clone().unwrap());
        Self::init_bucket_nodes(self_node_id, &cluster_state, bucket_node_assignments.clone(), node_connections.clone());
        Self::join_cluster(self_node_id, stream.try_clone().unwrap(), bucket_node_assignments.clone());
    }

    fn get_bucket_for_key(&self, key: &Key) -> BucketId {
        calculate_hash(key) % self.num_buckets
    }

    fn init_self_bucket_nodes(self_id: &NodeId,
                              num_buckets: u64,
                              bucket_nodes: Arc<Mutex<HashMap<BucketId, NodeId>>>,
    ) {
        let mut buckets = bucket_nodes.lock().unwrap();
        for bucket_id in 0..num_buckets {
            buckets.insert(bucket_id, self_id.clone());
        }
    }

    fn init_bucket_nodes(self_id: &NodeId,
                         cluster_state: &CmdResponseEnum,
                         self_bucket_nodes: Arc<Mutex<HashMap<BucketId, NodeId>>>,
                         self_node_connections: Arc<Mutex<HashMap<NodeId, Arc<Mutex<TcpStream>>>>>,
    ) {
        match cluster_state {
            CmdResponseEnum::ClusterState { buckets_to_nodes, nodes_to_ips } => {
                // opens connections to all the existing nodes
                buckets_to_nodes.iter().for_each(|(bucket, node)| {
                    info!("{self_id}.init_bucket_nodes: Bucket {bucket} is handled by {node}");
                    self_bucket_nodes.lock().unwrap().insert(*bucket, node.clone());
                });
                nodes_to_ips.iter().for_each(|(node, ip)| {
                    info!("{self_id}.init_bucket_nodes: Node {node} has ip: {ip}");
                    let connection = TcpStream::connect(ip).expect("Failed to connect to node");
                    self_node_connections.lock().unwrap().insert(node.clone(), Arc::new(Mutex::new(connection)));
                });
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

    fn join_cluster(self_node_id: &NodeId,
                    stream: TcpStream,
                    bucket_nodes: Arc<Mutex<HashMap<BucketId, NodeId>>>,
    ) {
        let mut writer = BufWriter::new(stream.try_clone().unwrap());
        let command = JoinCluster { node_id: self_node_id.to_string() };
        let mut command_str = serde_json::to_string(&command).unwrap();
        command_str.push('\n');
        writer.write_all(command_str.as_bytes()).unwrap();
        writer.flush().unwrap();

        let mut reader = BufReader::new(stream.try_clone().unwrap());
        let mut s = String::new();
        reader.read_line(&mut s).unwrap();
        info!("Received join cluster response: {s}");
        match serde_json::from_str(&s).unwrap() {
            UpdateClusterState { nodes_to_ips, buckets_to_nodes } => {
                let buckets_to_manage: Vec<BucketId> = buckets_to_nodes.iter()
                    .filter(|(_, node_id)| { node_id == &self_node_id })
                    .map(|(&key, _)| key)
                    .collect();
                info!("Node {self_node_id} will manage these buckets: {buckets_to_manage:?}");
                let mut bucket_nodes_cur = bucket_nodes.lock().unwrap();
                for bucket in buckets_to_manage {
                    bucket_nodes_cur.insert(bucket, self_node_id.to_string());
                }
            }
            _ => {
                warn!("Got incorrect response")
            }
        }
    }
}

fn calculate_hash<T: Hash>(t: &T) -> u64 {
    let mut s = DefaultHasher::new();
    t.hash(&mut s);
    s.finish()
}
