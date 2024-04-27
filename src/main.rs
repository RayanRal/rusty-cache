mod server {
    pub mod listener;
    pub mod local_test;

    pub mod cache;

    mod control_plane;

    mod requests;

    mod commands;

    pub mod cluster;
}


use clap::{Parser};
use crate::server::cache::Cache;
use log::{info, LevelFilter};
use env_logger::Builder;
use crate::server::cluster::Cluster;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    run_mode: String,
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
    let self_id = String::from("Server 1");
    // TODO: need to add command to connect to other servers or start standalone
    let cluster_status = Cluster::new(num_buckets, self_id);

    match cli.run_mode.as_str() {
        "server" => {
            info!("Running in server mode.");
            server::listener::start_server(cache, cluster_status, client_port, server_port);
        }
        "join" => {
            info!("Server mode, joining another server:");
        }
        "test" => {
            info!("Running cache testing mode.");
            server::local_test::run_test_mode(cache, cluster_status);
        }
        _ => {
            panic!("Invalid run mode. Please use 'server' or 'test'.");
        }
    }
}
