use std::{env};
use std::io::{stdin, Write};
use std::net::{Shutdown, TcpStream};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        println!("Usage: {} <ip> <port>", args[0]);
        return;
    }

    // let ip = &args[1];
    // let port = &args[2];
    // let mut stream = TcpStream::connect(format!("{ip}:{port}"))?;

    loop {
        // TODO: have to find a way to keep stream opened, instead of reopening each time
        let mut stream = TcpStream::connect("127.0.0.1:7878").expect("Failed to connect to server");

        println!("Send the command to server: set, get, exists, exit");

        let mut input = String::new();
        stdin().read_line(&mut input).expect("Failed to read line");

        stream.write_all(input.trim().as_bytes()).expect("Failed to send message");
        stream.shutdown(Shutdown::Both).unwrap();
    }
}