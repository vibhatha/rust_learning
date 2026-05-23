use std::net::TcpListener;

pub enum ServerStatus {
    /// Server Status
    /// At Success, provides a TcpListener
    /// At Failure, provides an error message 
    Success(TcpListener),
    Error(String),
}

pub struct WebServer {
    /// A simple webserver configuration
    host: String,
    port: String,
}

impl WebServer {

    pub fn new(host: String, port: String) -> WebServer {
        // Create a simple webserver
        WebServer {
            host,
            port,
        }
    }

    pub fn start_server(self) -> ServerStatus {
        // Start Web Server

        let address = format!("{}:{}", self.host, self.port);
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
}