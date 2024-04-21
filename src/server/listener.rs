use std::io::{BufRead, BufReader, BufWriter, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::{Arc, Mutex};
use rayon::ThreadPoolBuilder;
use crate::server::cache::Cache;
use crate::server::control_plane;

pub fn start_listener(cache: Cache) {
    let port = 7878;// &args[2];
    let listener = TcpListener::bind(format!("127.0.0.1:{port}")).unwrap();
    let shared_cache = Arc::new(Mutex::new(cache));
    let pool = ThreadPoolBuilder::new().num_threads(4).build().unwrap();

    for stream in listener.incoming() {
        let cache_clone = Arc::clone(&shared_cache);
        pool.spawn(move || {
            handle_connection(stream.unwrap(), cache_clone);
        });
    }
}

fn handle_connection(stream: TcpStream, cache: Arc<Mutex<Cache>>) {
    let stream_clone = stream.try_clone().unwrap();
    let mut reader = BufReader::new(stream);
    let mut writer = BufWriter::new(stream_clone);
    loop {
        let mut s = String::new();
        reader.read_line(&mut s).unwrap();
        println!("Got command: {s}");

        let mut cache = cache.lock().unwrap();
        control_plane::process_command(&s, &mut cache);

        writer.write("Ack\n".as_bytes()).unwrap();
        writer.flush().unwrap();
    }
}
