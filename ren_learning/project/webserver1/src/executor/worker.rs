use std::{sync::{Arc, Mutex, mpsc},
    thread};

use crate::executor::thread_pool::Job;

pub struct Worker {
    pub id: usize,
    pub thread: thread::JoinHandle<()>,
}

impl Worker {
    /// Create a new Worker.
    ///
    /// Each worker has a unique id.
    /// Each worker has a specific function to be called.
    ///
    /// # Panics
    ///
    /// The `new` function will panic if a valid job cannot be recieved.
    pub fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        let thread = thread::spawn(move || {
            loop {
                let message = receiver.lock().unwrap().recv();
                match message {
                    Ok(job) => {
                        println!("Worker {id} got a job; executing");

                        job();
                    }
                    Err(_) => {
                        println!("Worker {id} disconnected; shutting down.");
                        break;
                    }
                }
            }
        });

        Worker {id, thread}
    }
}
