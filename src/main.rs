mod server {
    pub mod listener;
    pub mod test_mode;

    pub mod cache;

    mod control_plane;

    mod commands;
}


use clap::{Parser};
use crate::server::cache::Cache;
use log::{info, LevelFilter};
use env_logger::Builder;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    run_mode: String,

    nodes: Vec<String>,
}


fn main() {
    Builder::new()
        .filter_level(LevelFilter::Debug)
        .init();

    let cli = Cli::parse();
    let cache = Cache::new();

    match cli.run_mode.as_str() {
        "server" => {
            info!("Running in server mode.");
            server::listener::start_listener(cache);
        }
        "test" => {
            info!("Running cache testing mode.");
            server::test_mode::run_test_mode(cache);
        }
        _ => {
            panic!("Invalid run mode. Please use 'server' or 'test'.");
        }
    }
}
