use webserver1::executor::thread_pool::ThreadPool;
use webserver1::network::server::WebServer;
use webserver1::network::server::ServerStatus;
use webserver1::handler::simple_handler::ConnectionHandler;


fn main() {
    let host = String::from("127.0.0.1");
    let port = String::from("7878");
    let webserver = WebServer::new(host, port);
    let status: ServerStatus = webserver.start_server();
    let num_threads: usize = 4;
    let pool = ThreadPool::new(num_threads);
    const MAX_REQUESTS: usize = 5;
    println!("Note: This WebServer Will Shutdown after {} requests", MAX_REQUESTS);

    match status {
        ServerStatus::Success(listener) => {
            println!("Server Ready to accept connections...");
            for stream in listener.incoming().take(MAX_REQUESTS) {
                let stream = stream.unwrap();
                pool.execute(|| {
                    match ConnectionHandler::handle(stream) {
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

