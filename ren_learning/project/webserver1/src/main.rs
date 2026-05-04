use log::debug;

use std::{
    fs,
    io::{BufReader, prelude::*},
    net::{TcpListener, TcpStream}
};

enum ServerStatus {
    Success(TcpListener),
    Error(String),
}

fn start_server(host: &String, port: &String) -> ServerStatus {
    // Good if we can validate the host and port
    let address = format!("{}:{}", host, port);
    match TcpListener::bind(&address) {
        Ok(listener) => {
            println!("Sever successfully connected to {address}");
            ServerStatus::Success(listener)
        }
        Err(error) => {
            let error_msg = format!(
                "Failed to start server at address {}: since {}", address, error
                );
            ServerStatus::Error(error_msg)
        }
    }
}

fn main() {
    let host = String::from("127.0.0.1");
    let port = String::from("7878");
    let status: ServerStatus = start_server(&host, &port);

    match status {
        ServerStatus::Success(listener) => {
            println!("Server Ready to accept connections...");
            for stream in listener.incoming() {
                let stream = stream.unwrap();
                handle_connection(stream);
            }
        }
        ServerStatus::Error(msg) => {
            eprintln!("Server setup failed: {}", msg);
        }
    }
    
}


fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&stream);
    let http_request: Vec<_> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();

    debug!("Received HTTP Request: {:#?}", http_request);

    let status_line = "HTTP/1.1 200 OK";
    let contents = fs::read_to_string("data/hello.html").unwrap();
    let length = contents.len();

    let response =
        format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

    stream.write_all(response.as_bytes()).unwrap();
}
