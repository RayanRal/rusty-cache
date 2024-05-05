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
    GetKeysForBucket {
        bucket_id: BucketId,
    },
}

// impl CommandsEnum {
//     pub(crate) fn serialize(&self) -> String {
//         match self {
//             JoinCluster { node_id } => { format!("join {node_id}") }
//             LeaveCluster { node_id } => { format!("leave {node_id}") }
//             GetClusterState {} => { "get_cluster_state".to_string() }
//             GetKeysForBucket { bucket_id } => format!("get_keys {bucket_id}")
//         }
//     }
// }


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
}

// #[derive(Debug, Serialize, Deserialize)]
// pub struct OkResponse {}
// 
// #[derive(Debug, Serialize, Deserialize)]
// pub struct ClusterState {
//     pub nodes_to_ips: HashMap<NodeId, SocketAddr>,
//     pub buckets_to_nodes: HashMap<BucketId, NodeId>,
// }
// 
// #[derive(Debug, Serialize, Deserialize)]
// pub struct KeysListResponse {
//     pub keys: Vec<Key>,
// }

// impl CmdResponse for OkResponse {
//     // fn serialize(&self) -> String {
//     //     String::from("OK")
//     // }
// }
// 
// impl CmdResponse for ClusterState {
//     // fn serialize(&self) -> String {
//     //     String::from("Cluster is ok")
//     // }
// }
// 
// impl CmdResponse for KeysListResponse {
//     // fn serialize(&self) -> String {
//     //     self.keys.join(", ")
//     // }
// }

// pub fn deserialize_command(input: String) -> CommandsEnum {
//     let parts: Vec<&str> = input.split_whitespace().collect();
//     let command = parts.first();
// 
//     match command {
//         Some(&"get_cluster_state") => {
//             GetClusterState {}
//         }
//         Some(&"join") => {
//             let node_id = String::from(parts[1]);
//             JoinCluster { node_id }
//         }
//         Some(&"get_keys") => {
//             let bucket_id: BucketId = parts[1].parse::<u64>().unwrap();
//             GetKeysForBucket {
//                 bucket_id
//             }
//         }
//         Some(&"leave") => {
//             let node_id = String::from(parts[1]);
//             LeaveCluster { node_id }
//         }
//         cmd => {
//             panic!("Command {cmd:?} not found.");
//         }
//     }
// }