use std::{env, io};
use std::io::{BufRead, BufReader, BufWriter, Write};
use std::net::{TcpStream};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        println!("Usage: {} <ip> <port>", args[0]);
        return;
    }

    // let ip = &args[1];
    // let port = &args[2];
    // let mut stream = TcpStream::connect(format!("{ip}:{port}"))?;

    let mut stream = TcpStream::connect("127.0.0.1:7878").expect("Failed to connect to server");
    let stream_clone = stream.try_clone().unwrap();
    let mut reader = BufReader::new(stream);
    let mut writer = BufWriter::new(stream_clone);
    loop {
        println!("Send the command to server: set, get, exists, exit");
        let mut request = String::new();
        io::stdin().read_line(&mut request).unwrap();
        writer.write(request.as_bytes()).unwrap();
        writer.flush().unwrap();

        let mut response = String::new();
        reader.read_line(&mut response).unwrap();
        println!("Server response: {}", response.trim());
    }
}