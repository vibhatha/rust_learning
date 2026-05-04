use std::{
    fs,
    io::{BufReader, prelude::*},
    net::{TcpListener, TcpStream},
    thread,
    time::Duration,
};

enum ServerStatus {
    Success(TcpListener),
    Error(String),
}

use webserver1::ThreadPool;


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

fn thread_pool_init(num_threads: usize) -> ThreadPool {
    ThreadPool::new(num_threads)
}

fn main() {
    let host = String::from("127.0.0.1");
    let port = String::from("7878");
    let status: ServerStatus = start_server(&host, &port);
    let num_threads: usize = 4;
    let pool = thread_pool_init(num_threads);
    const MAX_REQUESTS: usize = 5;
    println!("Note: This WebServer Will Shutdown after {} requests", MAX_REQUESTS);

    match status {
        ServerStatus::Success(listener) => {
            println!("Server Ready to accept connections...");
            for stream in listener.incoming().take(MAX_REQUESTS) {
                let stream = stream.unwrap();
                pool.execute(|| {
                    match handle_connection(stream) {
                        Ok(()) => {
                            println!("Connection successfully handled!");
                        }
                        Err(error) => {
                            eprintln!("Connection handling failed: {}", error);
                        }
                    }
                });
            }
            println!("Shutting Down!");
        }
        ServerStatus::Error(msg) => {
            eprintln!("Server setup failed: {}", msg);
        }
    }
}


fn handle_connection(mut stream: TcpStream) -> std::result::Result<(), Box<dyn std::error::Error>> {
    let buf_reader = BufReader::new(&stream);
    let request_line = buf_reader.lines().next().ok_or("Empty request")??;

    let (status_line, filename) = match &request_line[..] {
        "GET / HTTP/1.1" => ("HTTP/1.1 200 OK", "data/hello.html"),
        "GET /sleep HTTP/1.1" => {
            thread::sleep(Duration::from_secs(5));
            ("HTTP/1.1 200 OK", "data/hello_sleep.html")
        }
        _ => ("HTTP/1.1 404 NOT FOUND", "data/404.html"),
    };

    let contents = fs::read_to_string(filename)?;
    let length = contents.len();

    let response =
        format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

    stream.write_all(response.as_bytes())?;
    Ok(())
}
