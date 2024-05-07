use std::net::TcpStream;
use log::warn;
use crate::server::cluster::Cluster;
use crate::server::commands::{CmdResponseEnum, CommandsEnum};

pub fn process_cluster_command(command: CommandsEnum, cluster: &mut Cluster, connection_stream: TcpStream) -> CmdResponseEnum {
    match command {
        CommandsEnum::JoinCluster { node_id: new_node_id } => {
            cluster.add_node_connection(new_node_id.clone(), connection_stream.try_clone().unwrap());
            let mut nodes_to_ips = cluster.get_cluster_node_ips();
            nodes_to_ips.insert(cluster.self_node_id.to_string(), connection_stream.local_addr().unwrap());
            cluster.redistribute_buckets();
            let buckets_to_nodes = cluster.get_bucket_node_assignments();

            let update_cluster_cmd = CommandsEnum::UpdateClusterState {
                nodes_to_ips: nodes_to_ips.clone(),
                buckets_to_nodes: buckets_to_nodes.clone(),
            };
            cluster.notify_cluster_nodes(update_cluster_cmd);
            CmdResponseEnum::ClusterState { nodes_to_ips, buckets_to_nodes }
        }
        CommandsEnum::GetClusterState {} => {
            let nodes_to_ips = cluster.get_cluster_node_ips();
            let buckets_to_nodes = cluster.get_bucket_node_assignments();
            CmdResponseEnum::ClusterState { nodes_to_ips, buckets_to_nodes }
        }
        CommandsEnum::UpdateClusterState { nodes_to_ips, buckets_to_nodes } => {
            cluster.update_cluster_state(nodes_to_ips, buckets_to_nodes);
            CmdResponseEnum::Ok
        }
        CommandsEnum::LeaveCluster { node_id } => {
            warn!("Node {node_id} leaves the cluster");
            // TODO: remove node from node_connections, close connection
            CmdResponseEnum::Ok
        }
    }
}