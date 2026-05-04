use std::{
    error::{Error},
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
                match handle_connection(stream) {
                    Ok(()) => {
                        println!("Connection successfully handled!");
                    }
                    Err(error) => {
                        eprintln!("Connection handling failed: {}", error);
                    }
                }
            }
        }
        ServerStatus::Error(msg) => {
            eprintln!("Server setup failed: {}", msg);
        }
    }
}


fn handle_connection(mut stream: TcpStream) -> Result<(), Box<dyn Error>> {
    let buf_reader = BufReader::new(&stream);
    let request_line = buf_reader.lines().next().ok_or("Empty request")??;

    let (status_line, filename) = if request_line == "GET / HTTP/1.1" {
        ("HTTP/1.1 200 OK", "data/hello.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND", "data/404.html")
    };

    let contents = fs::read_to_string(filename)?;
    let length = contents.len();

    let response =
        format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

    stream.write_all(response.as_bytes()).unwrap();
    Ok(())
}
