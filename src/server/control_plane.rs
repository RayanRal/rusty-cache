use std::net::TcpStream;
use log::warn;
use crate::server::cache::Cache;
use crate::server::cluster::Cluster;
use crate::server::commands::{CmdResponseEnum, CommandsEnum};
use crate::server::requests::{ReqResponseEnum, RequestsEnum};

pub fn process_client_request(request: RequestsEnum, cache: &mut Cache, cluster: &mut Cluster) -> ReqResponseEnum {
    match request {
        RequestsEnum::Put { key, value, ttl } => {
            let is_key_local = cluster.is_key_local(&key);
            if is_key_local {
                cache.put(&key, &value, ttl);
                ReqResponseEnum::Put {}
            } else {
                let _target_node = cluster.get_node_for_key(&key);
                // TODO: redirect put request to another node if necessary
                ReqResponseEnum::Put {}
            }
        }
        RequestsEnum::Get { key } => {
            let is_key_local = cluster.is_key_local(&key);
            if is_key_local {
                let value = cache.get(&key);
                ReqResponseEnum::Get {
                    key,
                    value,
                }
            } else {
                let _target_node = cluster.get_node_for_key(&key);
                // send a Get request to appropriate node
                ReqResponseEnum::CommandNotFound {}
            }
        }
        RequestsEnum::Exists { key } => {
            let is_key_local = cluster.is_key_local(&key);
            if is_key_local {
                let exists = cache.exists(&key);
                ReqResponseEnum::Exists { exists }
            } else {
                let _target_node = cluster.get_node_for_key(&key);
                // send an Exists request to appropriate node
                ReqResponseEnum::CommandNotFound {}
            }
        }
        RequestsEnum::Exit {} => {
            warn!("Received EXIT command. Wrapping up.");
            panic!("Received EXIT command");
        }
    }
}

pub fn process_cluster_command(command: CommandsEnum, cluster: &mut Cluster, connecting_node: TcpStream) -> CmdResponseEnum {
    match command {
        CommandsEnum::JoinCluster { node_id } => {
            cluster.add_node_connection(node_id, connecting_node);
            CmdResponseEnum::Ok
        }
        CommandsEnum::GetClusterState { .. } => {
            let nodes_to_ips = cluster.get_connected_nodes_ips();
            let buckets_to_nodes = cluster.get_bucket_node_assignments();
            CmdResponseEnum::ClusterState { nodes_to_ips, buckets_to_nodes }
        }
        CommandsEnum::GetKeysForBucket { bucket_id } => {
            let keys = cluster.get_all_keys_for_bucket(bucket_id);
            CmdResponseEnum::KeysList { keys }
        }
        CommandsEnum::LeaveCluster { node_id } => { CmdResponseEnum::Ok }
    }
}