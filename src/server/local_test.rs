use std::io;
use log::info;
use crate::server::cache::Cache;
use crate::server::{requests, control_plane};


pub fn run_test_mode(mut cache: Cache) {
    loop {
        info!("Enter command: set, get, exists, exit");
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        let command = requests::deserialize_request(input);
        control_plane::process_client_request(command, &mut cache);
    }
}
