mod server {
    pub mod listener;
    pub mod local_test;

    pub mod cache;

    mod control_plane;

    mod requests;

    mod commands;

    pub mod cluster;
}


use std::net::IpAddr;
use std::str::FromStr;
use clap::{Parser};
use crate::server::cache::Cache;
use log::{info, LevelFilter};
use env_logger::Builder;
use crate::server::cluster::{Cluster, NodeId};
use rand::distributions::{Alphanumeric, DistString};


#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    run_mode: String,
    nodes: Option<Vec<String>>,
}


fn main() {
    Builder::new()
        .filter_level(LevelFilter::Debug)
        .init();

    let cli = Cli::parse();
    let cache = Cache::new();
    let client_port: u32 = 7878;
    let server_port: u32 = 9090;
    let num_buckets = 16;
    let self_id = format!("node-{}", generate_node_id());
    // if ip of node to connect is provided, parse it and try to connect
    let main_node_ip = IpAddr::from_str("").ok();
    info!("Starting with params:
     - client port: {client_port};
     - server port: {server_port};
     - num buckets: {num_buckets};
     - id: {self_id};
     - main node ip: {main_node_ip:?};
    ");

    let cluster_state = Cluster::new(num_buckets, self_id, main_node_ip);

    match cli.run_mode.as_str() {
        "server" => {
            info!("Running in server mode.");
            server::listener::start_server(cache, cluster_state, client_port, server_port);
        }
        "test" => {
            info!("Running cache testing mode.");
            server::local_test::run_test_mode(cache, cluster_state);
        }
        _ => {
            panic!("Invalid run mode. Please use 'server' or 'test'.");
        }
    }
}

fn generate_node_id() -> NodeId {
    Alphanumeric.sample_string(&mut rand::thread_rng(), 5)
}