use std::io::{BufRead, BufReader};
use std::net::{TcpListener, TcpStream};
use rayon::{
    ThreadPoolBuilder};

pub fn start_listener() {
    let port = 7878;// &args[2];
    let listener = TcpListener::bind(format!("127.0.0.1:{port}")).unwrap();
    let pool = ThreadPoolBuilder::new().num_threads(4).build().unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        pool.spawn(|| {
            handle_connection(stream)
        });
    }
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let http_request: Vec<_> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();

    println!("Got request: {:#?}", http_request);
}
