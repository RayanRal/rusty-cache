use std::net::TcpStream;
use log::warn;
use crate::server::cache::Cache;
use crate::server::commands::{ClusterState, CmdResponse, CommandsEnum, KeysListResponse, OkResponse};
use crate::server::{requests};
use crate::server::cluster::Cluster;
use crate::server::requests::{CommandNotFoundResponse, RequestsEnum};

pub fn process_client_request(request: RequestsEnum, cache: &mut Cache, cluster: &mut Cluster) -> Box<dyn requests::ReqResponse> {
    match request {
        RequestsEnum::Put { key, value, ttl } => {
            let is_key_local = cluster.is_key_local(&key);
            if is_key_local {
                cache.put(&key, &value, ttl);
                let response = requests::PutResponse {};
                Box::new(response)
            } else {
                let target_node = cluster.get_node_for_key(&key);
                // TODO: redirect put request to another node if necessary
                let response = requests::PutResponse {};
                Box::new(response)
            }
        }
        RequestsEnum::Get { key } => {
            let is_key_local = cluster.is_key_local(&key);
            if is_key_local {
                let value = cache.get(&key);
                let response = requests::GetResponse {
                    key,
                    value,
                };
                Box::new(response)
            } else {
                let target_node = cluster.get_node_for_key(&key);
                // send a Get request to appropriate node
                let response = CommandNotFoundResponse {};
                Box::new(response)
            }
        }
        RequestsEnum::Exists { key } => {
            let is_key_local = cluster.is_key_local(&key);
            if is_key_local {
                let exists = cache.exists(&key);
                let response = requests::ExistsResponse { exists };
                Box::new(response)
            } else {
                let target_node = cluster.get_node_for_key(&key);
                // send an Exists request to appropriate node
                let response = CommandNotFoundResponse {};
                Box::new(response)
            }
        }
        RequestsEnum::Exit {} => {
            warn!("Received EXIT command. Wrapping up.");
            panic!("Received EXIT command");
        }
    }
}

pub fn process_cluster_command(command: CommandsEnum, cluster: &mut Cluster, connecting_node: TcpStream) -> Box<dyn CmdResponse> {
    match command {
        CommandsEnum::JoinCluster { node_id } => {
            cluster.add_node_connection(node_id, connecting_node);
            Box::new(OkResponse {})
        }
        CommandsEnum::GetClusterState { .. } => {
            let nodes_to_ips = cluster.get_connected_nodes_ips();
            let buckets_to_nodes = cluster.get_bucket_node_assignments();
            Box::new(ClusterState { nodes_to_ips, buckets_to_nodes })
        }
        CommandsEnum::GetKeysForBucket { bucket_id } => {
            let keys = cluster.get_all_keys_for_bucket(bucket_id);
            Box::new(KeysListResponse { keys })
        }
        CommandsEnum::LeaveCluster { node_id } => { Box::new(OkResponse {}) }
    }
}