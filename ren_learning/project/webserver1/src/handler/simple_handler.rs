use std::{
    fs,
    io::{BufReader, prelude::*},
    net::{TcpStream},
    thread,
    time::Duration,
};

pub struct ConnectionHandler { }

impl ConnectionHandler {

	pub fn handle(mut stream: TcpStream) -> std::result::Result<(), Box<dyn std::error::Error>> {
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
}