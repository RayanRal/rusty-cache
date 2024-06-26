use std::io::{BufRead, BufReader, BufWriter, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
use log::{info, warn};
use rayon::ThreadPoolBuilder;
use crate::server::cache::Cache;
use crate::server::{cluster_command_processing, user_request_processing};
use crate::server::cluster::Cluster;

const CLIENT_THREADS: usize = 1;
const SERVER_THREADS: usize = 3;

pub fn start_server(cache: Cache, 
                    cluster: Cluster, 
                    client_port: u32, 
                    server_port: u32
) {
    let client_listener = TcpListener::bind(format!("127.0.0.1:{client_port}")).unwrap();
    let server_listener = TcpListener::bind(format!("127.0.0.1:{server_port}")).unwrap();

    let cluster_state = Arc::new(Mutex::new(cluster));
    let client_cluster = Arc::clone(&cluster_state);
    let server_cluster = Arc::clone(&cluster_state);

    let shared_cache = Arc::new(Mutex::new(cache));

    let client_threads = thread::spawn(move || {
        let client_pool = ThreadPoolBuilder::new().num_threads(CLIENT_THREADS).build().unwrap();
        for stream in client_listener.incoming() {
            let client_cache_clone_per_connection = Arc::clone(&shared_cache);
            let client_cluster_status_per_connection = Arc::clone(&client_cluster);
            client_pool.spawn(move || {
                handle_client_connection(stream.unwrap(), client_cluster_status_per_connection, client_cache_clone_per_connection);
            });
        }
    });
    let server_threads = thread::spawn(move || {
        // if new nodes never disconnect, we need at least number of threads = number of expected nodes
        let server_pool = ThreadPoolBuilder::new().num_threads(SERVER_THREADS).build().unwrap();
        for stream in server_listener.incoming() {
            let server_cluster_status_per_connection = Arc::clone(&server_cluster);
            server_pool.spawn(move || {
                handle_server_connection(stream.unwrap(), server_cluster_status_per_connection);
            });
        }
    });

    client_threads.join().unwrap();
    server_threads.join().unwrap();
}

fn handle_client_connection(stream: TcpStream, 
                            cluster: Arc<Mutex<Cluster>>, 
                            cache: Arc<Mutex<Cache>>
) {
    let mut reader = BufReader::new(stream.try_clone().unwrap());
    let mut writer = BufWriter::new(stream.try_clone().unwrap());
    loop {
        let mut s = String::new();
        reader.read_line(&mut s).unwrap();
        info!("Received client request: {s}");
        match serde_json::from_str(&s) {
            Ok(request) => {
                let mut cache = cache.lock().unwrap();
                let mut cluster = cluster.lock().unwrap();
                let response = user_request_processing::process_client_request(request, &mut cache, &mut cluster);
                let mut response_str = serde_json::to_string(&response).unwrap();
                response_str.push('\n');

                writer.write_all(response_str.as_bytes()).unwrap();
                writer.flush().unwrap();
            }
            Err(_e) => {
                warn!("Couldn't parse client request: {s}")
            }
        }
    }
}

fn handle_server_connection(stream: TcpStream, cluster: Arc<Mutex<Cluster>>) {
    let mut reader = BufReader::new(stream.try_clone().unwrap());
    let mut writer = BufWriter::new(stream.try_clone().unwrap());
    loop {
        let mut s = String::new();
        match reader.read_line(&mut s) {
            Ok(usize) => {
                if usize == 0 {
                    continue;
                }

                info!("Received cluster command: {s}");
                match serde_json::from_str(&s) {
                    Ok(command) => {
                        let mut cluster = cluster.lock().unwrap();
                        let response = cluster_command_processing::process_cluster_command(command, &mut cluster, stream.try_clone().unwrap());
                        let mut response_str = serde_json::to_string(&response).unwrap();
                        response_str.push('\n');

                        writer.write_all(response_str.as_bytes()).unwrap();
                        writer.flush().unwrap();
                    }
                    Err(_e) => {
                        warn!("Couldn't parse command: {s}")
                    }
                }
            }
            Err(_) => {
                warn!("Error reading from stream")
            }
        }
        thread::sleep(Duration::from_millis(100));
    }
}
