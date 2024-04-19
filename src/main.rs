mod server {
    pub mod server;
    pub mod test_mode;

    pub mod cache;
}


use std::env;

fn main() {
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
}
