use std::io;
use log::info;
use crate::server::cache::Cache;
use crate::server::control_plane;
use crate::server::cluster::Cluster;


pub fn run_test_mode(mut cache: Cache, mut cluster: Cluster) {
    loop {
        info!("Enter command: set, get, exists, exit");
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        let command = serde_json::from_str(&input).unwrap();
        control_plane::process_client_request(command, &mut cache, &mut cluster);
    }
}
