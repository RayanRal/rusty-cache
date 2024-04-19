use std::net::TcpListener;

pub fn start_server() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
}
