use std::io::{BufRead, BufReader, BufWriter, Write};
use std::net::{IpAddr, TcpListener, TcpStream};
use std::sync::{Arc, Mutex};
use std::thread;
use log::info;
use rayon::ThreadPoolBuilder;
use crate::server::cache::Cache;
use crate::server::{requests, control_plane, commands};
use crate::server::cluster::Cluster;

pub fn start_server(cache: Cache, cluster: Cluster, client_port: u32, server_port: u32) {
    let client_listener = TcpListener::bind(format!("127.0.0.1:{client_port}")).unwrap();
    let server_listener = TcpListener::bind(format!("127.0.0.1:{server_port}")).unwrap();

    let cluster_state = Arc::new(Mutex::new(cluster));
    let client_cluster = Arc::clone(&cluster_state);
    let server_cluster = Arc::clone(&cluster_state);

    let shared_cache = Arc::new(Mutex::new(cache));
    let client_cache_clone = Arc::clone(&shared_cache);

    thread::spawn(move || {
        let client_pool = ThreadPoolBuilder::new().num_threads(1).build().unwrap();
        for stream in client_listener.incoming() {
            let client_cache_clone_per_connection = Arc::clone(&client_cache_clone);
            let client_cluster_status_per_connection = Arc::clone(&client_cluster);
            client_pool.spawn(move || {
                handle_client_connection(stream.unwrap(), client_cluster_status_per_connection, client_cache_clone_per_connection);
            });
        }
    });
    thread::spawn(move || {
        let server_pool = ThreadPoolBuilder::new().num_threads(2).build().unwrap();
        for stream in server_listener.incoming() {
            let server_cluster_status_per_connection = Arc::clone(&server_cluster);
            server_pool.spawn(move || {
                handle_server_connection(stream.unwrap(), server_cluster_status_per_connection);
            });
        }
    });
    // if it's a secondary node, it needs also to connect to root server and send "join cluster" command
}

fn get_node_id(stream: &TcpStream) -> String {
    stream.peer_addr().unwrap().ip().to_string()
}

fn handle_client_connection(stream: TcpStream, cluster: Arc<Mutex<Cluster>>, cache: Arc<Mutex<Cache>>) {
    let stream_clone = stream.try_clone().unwrap();
    let mut reader = BufReader::new(stream);
    let mut writer = BufWriter::new(stream_clone);
    loop {
        let mut s = String::new();
        reader.read_line(&mut s).unwrap();
        info!("Received client request: {s}");
        let request = requests::deserialize_request(s);

        let mut cache = cache.lock().unwrap();
        let mut cluster = cluster.lock().unwrap();
        let response = control_plane::process_client_request(request, &mut cache, &mut cluster);
        let mut response_str = response.serialize();
        response_str.push('\n');

        writer.write_all(response_str.as_bytes()).unwrap();
        writer.flush().unwrap();
    }
}

fn handle_server_connection(stream: TcpStream, cluster: Arc<Mutex<Cluster>>) {
    let stream_clone = stream.try_clone().unwrap();
    let mut reader = BufReader::new(stream);
    // let mut writer = BufWriter::new(stream_clone);
    loop {
        let mut cluster = cluster.lock().unwrap();
        let mut s = String::new();
        let stream_clones = stream_clone.try_clone().unwrap();
        reader.read_line(&mut s).unwrap();
        info!("Received cluster command: {s}");
        let command = commands::deserialize_command(s);

        control_plane::process_cluster_command(command, &mut cluster, stream_clones);
        // todo: for now let's not bother about responding to another node
        // let mut response_str = response.serialize();
        // response_str.push('\n');
        // 
        // writer.write_all(response_str.as_bytes()).unwrap();
        // writer.flush().unwrap();
    }
}

fn handle_cluster_join(main_node_ip: IpAddr) {
    
}