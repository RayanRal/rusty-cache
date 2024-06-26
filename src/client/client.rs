use std::{env, io};
use std::io::{BufRead, BufReader, BufWriter, Write};
use std::net::{TcpStream};
use env_logger::Builder;
use log::{error, info, LevelFilter};

fn main() {
    Builder::new()
        .filter_level(LevelFilter::Debug)
        .init();

    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        error!("Usage: {} <ip> <port>", args[0]);
        return;
    }

    let ip = &args[1];
    let port = &args[2];
    let stream = TcpStream::connect(format!("{ip}:{port}")).expect("Failed to connect to server");

    let mut reader = BufReader::new(stream.try_clone().unwrap());
    let mut writer = BufWriter::new(stream.try_clone().unwrap());
    loop {
        info!("Send the command to server in JSON: Put, Get, Exists, Exit");
        let mut request = String::new();
        io::stdin().read_line(&mut request).unwrap();
        // TODO: provide an easier interface to provide commands (not json)
        writer.write_all(request.as_bytes()).unwrap();
        writer.flush().unwrap();

        let mut response = String::new();
        reader.read_line(&mut response).unwrap();
        info!("Server response: {}", response.trim());
    }
}