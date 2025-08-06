use core::{ops::FnOnce, result::Result::Err};
use std::{sync::{
    mpsc,
    Arc,
    Mutex
}, thread};
pub struct ThreadPool{
    workers: Vec<Worker>,
    sender: Option<mpsc::Sender<Job>>,
}
// Defines a type alias 'Job' for a boxed closure that takes no arguments, returns nothing,
// can be sent across threads (Send), and has a 'static lifetime.
// This is commonly used for storing tasks to be executed by a thread pool.
type Job = Box<dyn FnOnce() + Send + 'static>;

impl ThreadPool {
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0, "Thread pool size must be greater than zero.");
        let (sender, reciever) = mpsc::channel();
        let reciever = Arc::new(Mutex::new(reciever));
        let mut workers = Vec::with_capacity(size);
        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&reciever)));
        }
        ThreadPool { workers, sender: Some(sender) }
    }
    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);
        if let Some(sender) = &self.sender {
            sender.send(job).unwrap();
        }
    }
}

impl Drop for ThreadPool {
    // The Drop trait allows you to customize what happens when a ThreadPool instance goes out of scope.
    fn drop(&mut self) {
        // Take the sender out of the ThreadPool, replacing it with None.
        drop(self.sender.take());
        // Iterate over all workers in the pool, draining them (removing all elements from the vector).
        for worker in &mut self.workers.drain(..) {
            // Print a message indicating which worker is being shut down.
            println!("Shutting down worker {}", worker.id);
            // Wait for the worker's thread to finish execution before continuing.
            // `join()` blocks until the thread has exited.
            // `unwrap()` will panic if the thread panicked.
            worker.thread.join().unwrap();
        }
        // After this loop, all worker threads have been joined and cleaned up.
    }
}

struct Worker {
    id: usize,
    thread: thread::JoinHandle<()>,
}

impl Worker {
    fn new(id: usize, reciever: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        let thread = thread::spawn(move || {
            loop {
                let job = reciever.lock().unwrap().recv();
                match job {
                    Ok(job) => {
                        println!("Worker {} got a job; executing.", id);
                        job();
                }
                Err(_) => {
                    println!("Worker {} disconnected; shutting down.", id);
                    break;
                }
            }
        }});
        Worker { id, thread}
    }
}