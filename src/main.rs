mod server {
    pub mod listener;
    pub mod test_mode;

    pub mod cache;

    pub mod control_plane;

    pub mod commands;

    pub mod evictor;
}


use std::env;
use crate::server::cache::Cache;
use log::{info, LevelFilter};
use env_logger::Builder;

fn main() {
    Builder::new()
        .filter_level(LevelFilter::Debug)
        .init();

    let args: Vec<String> = env::args().collect();
    let cache = Cache::new();
    if args.len() < 2 {
        panic!("Usage: {} <run_mode>", args[0]);
    }

    let run_mode = &args[1];
    match run_mode.as_str() {
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
