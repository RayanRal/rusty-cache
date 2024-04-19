
mod server {
    pub mod server;
    pub mod test_mode;

    pub mod cache;
}


use std::net::TcpListener;
use std::{env, io};
// use server::server;
// use server::test_mode;

fn main() {
    println!("Select");

    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Usage: {} <run_mode>", args[0]);
        return;
    }

    let run_mode = &args[1];
    match run_mode.as_str() {
        "server" => {
            println!("Running in server mode.");
            server::server::start_server();
        }
        "test" => {
            println!("Running cache testing mode.");
            server::test_mode::run_test_mode();
        }
        _ => {
            println!("Invalid run mode. Please use 'server' or 'test'.");
        }
    }
    
    // core module - has methods to set / get / update cache values
    // input module, with 2 implementations:
    // - reads from console
    // - reads from network

    // normal running mode
    // daemon process that listens to TCP connections
    // local cli - separate executable, connects to local server

    // test running mode: local, reads / writes to terminal, purely to test HashMap
}
