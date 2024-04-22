mod server {
    pub mod listener;
    pub mod test_mode;

    pub mod cache;

    pub mod control_plane;

    pub mod commands;
}


use std::env;
use crate::server::cache::Cache;
use log::LevelFilter;
use env_logger::Builder;

fn main() {
    Builder::new()
        .filter_level(LevelFilter::Debug)
        .init();

    let args: Vec<String> = env::args().collect();
    let cache = Cache::new();
    if args.len() < 2 {
        println!("Usage: {} <run_mode>", args[0]);
        return;
    }

    let run_mode = &args[1];
    match run_mode.as_str() {
        "server" => {
            println!("Running in server mode.");
            server::listener::start_listener(cache);
        }
        "test" => {
            println!("Running cache testing mode.");
            server::test_mode::run_test_mode(cache);
        }
        _ => {
            println!("Invalid run mode. Please use 'server' or 'test'.");
        }
    }
}
