use std::collections::HashMap;
use std::net::{SocketAddr};
use serde::{Deserialize, Serialize};
use crate::server::cache::Key;
use crate::server::cluster::{BucketId, NodeId};

#[derive(Debug, Serialize, Deserialize)]
pub enum CommandsEnum {
    JoinCluster {
        node_id: NodeId,
    },
    LeaveCluster {
        node_id: NodeId,
    },
    GetClusterState {},
    UpdateClusterState {
        nodes_to_ips: HashMap<NodeId, SocketAddr>,
        buckets_to_nodes: HashMap<BucketId, NodeId>,
    },
}

#[derive(Debug, Serialize, Deserialize)]
pub enum CmdResponseEnum {
    Ok,
    ClusterState {
        nodes_to_ips: HashMap<NodeId, SocketAddr>,
        buckets_to_nodes: HashMap<BucketId, NodeId>,
    },
    KeysList {
        keys: Vec<Key>,
    },
    ErrorProcessingCommand,
}
