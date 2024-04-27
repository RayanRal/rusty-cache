use crate::server::commands::CommandsEnum::{GetClusterState, GetKeysForBucket, JoinCluster, LeaveCluster};

pub enum CommandsEnum {
    JoinCluster {
        server_id: String,
    },
    LeaveCluster {},
    GetClusterState {},
    GetKeysForBucket {
        bucket_id: String,
    },
}


pub trait CmdResponse {
    fn serialize(&self) -> String;
}

pub struct OkResponse {}

pub struct ClusterState {
    // todo: for now it's useless, just to see that cluster exists
    pub node_ids: Vec<String>
}

pub struct KeysListResponse {
    pub keys: Vec<String>,
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
            let bucket_id = String::from("");
            GetKeysForBucket {
                bucket_id
            }
        }
        cmd => {
            panic!("Command {cmd} not found.");
        }
    }
}