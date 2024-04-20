use std::io;
use crate::server::cache::Cache;
use crate::server::control_plane;


pub fn run_test_mode() {
    let mut cache = Cache::new();

    loop {
        println!("Enter command: set, get, exists, exit");
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        control_plane::process_command(&input, &mut cache);
    }
}
