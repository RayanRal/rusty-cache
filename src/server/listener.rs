use std::io::{BufRead, BufReader, BufWriter, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::{Arc, Mutex};
use std::thread;
use log::info;
use rayon::ThreadPoolBuilder;
use crate::server::cache::Cache;
use crate::server::{requests, control_plane};
use crate::server::cluster::Cluster;

pub fn start_server(cache: Cache, cluster: Cluster, client_port: u32, server_port: u32) {
    let client_listener = TcpListener::bind(format!("127.0.0.1:{client_port}")).unwrap();
    let server_listener = TcpListener::bind(format!("127.0.0.1:{server_port}")).unwrap();

    let shared_cluster_status =  Arc::new(Mutex::new(cluster));
    let client_cluster_status = Arc::clone(&shared_cluster_status);
    let server_cluster_status = Arc::clone(&shared_cluster_status);

    let shared_cache = Arc::new(Mutex::new(cache));
    let client_cache_clone = Arc::clone(&shared_cache);


    thread::spawn(move || {
        let client_pool = ThreadPoolBuilder::new().num_threads(1).build().unwrap();
        for stream in client_listener.incoming() {
            let client_cache_clone_per_connection = Arc::clone(&client_cache_clone);
            let client_cluster_status_per_connection = Arc::clone(&client_cluster_status);
            client_pool.spawn(move || {
                handle_client_connection(stream.unwrap(), client_cluster_status_per_connection, client_cache_clone_per_connection);
            });
        }
    });
    for stream in server_listener.incoming() {
        let mut cluster_status =  server_cluster_status.lock().unwrap();
        let tcp_stream = stream.unwrap();
        let server_id = get_server_id(&tcp_stream);
        // TODO: current server responds with cluster status
        // TODO: current server spawns separate thread and calls bulk_put to transfer keys
        // add_server should happen only after new server caught up on current key status
        cluster_status.add_server(server_id, tcp_stream);
    }

}

fn get_server_id(stream: &TcpStream) -> String {
    stream.peer_addr().unwrap().ip().to_string()
}

fn handle_client_connection(stream: TcpStream, cluster: Arc<Mutex<Cluster>>, cache: Arc<Mutex<Cache>>) {
    let stream_clone = stream.try_clone().unwrap();
    let mut reader = BufReader::new(stream);
    let mut writer = BufWriter::new(stream_clone);
    loop {
        let mut s = String::new();
        reader.read_line(&mut s).unwrap();
        info!("Received request: {s}");
        let command = requests::deserialize_request(s);

        let mut cache = cache.lock().unwrap();
        let response = control_plane::process_client_request(command, &mut cache);
        let mut response_str = response.serialize();
        response_str.push('\n');

        writer.write_all(response_str.as_bytes()).unwrap();
        writer.flush().unwrap();
    }
}
