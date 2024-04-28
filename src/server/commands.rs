use std::collections::HashMap;
use std::net::IpAddr;
use crate::server::cache::Key;
use crate::server::cluster::{BucketId, NodeId};
use crate::server::commands::CommandsEnum::{GetClusterState, GetKeysForBucket, JoinCluster, LeaveCluster};

pub enum CommandsEnum {
    JoinCluster {
        server_id: NodeId,
    },
    LeaveCluster {},
    GetClusterState {},
    GetKeysForBucket {
        bucket_id: BucketId,
    },
}


pub trait CmdResponse {
    fn serialize(&self) -> String;
}

pub struct OkResponse {}

pub struct ClusterState {
    pub nodes: HashMap<NodeId, IpAddr>,
    pub nodes_to_buckets: HashMap<BucketId, NodeId>,
}

pub struct KeysListResponse {
    pub keys: Vec<Key>,
}

impl CmdResponse for OkResponse {
    fn serialize(&self) -> String {
        String::from("OK")
    }
}

impl CmdResponse for ClusterState {
    fn serialize(&self) -> String {
        String::from("Cluster is ok")
    }
}

impl CmdResponse for KeysListResponse {
    fn serialize(&self) -> String {
        self.keys.join(", ")
    }
}

pub fn deserialize_command(input: String) -> CommandsEnum {
    let input_to_match = input.as_str();
    match input_to_match {
        "join" => {
            let server_id = String::from("2");
            JoinCluster { server_id }
        }
        "leave" => {
            LeaveCluster {}
        }
        "state" => {
            GetClusterState {}
        }
        "keys" => {
            let bucket_id = 2;
            GetKeysForBucket {
                bucket_id
            }
        }
        cmd => {
            panic!("Command {cmd} not found.");
        }
    }
}