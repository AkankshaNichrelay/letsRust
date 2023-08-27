// in order to make our library crate our primary crate, we move main.rs into bin folder.

use std::{thread, sync::{mpsc, Arc, Mutex}};

pub struct ThreadPool {
    workers: Vec<Worker>,

    //sender if of type Sender from mpsc module and we are sending jobs across threads
    sender: mpsc::Sender<Job>,    
}  

struct Job;

impl ThreadPool {
    /// Create a new ThreadPool
    /// 
    /// The size is the number of threads in the pool.
    /// 
    /// # Panics
    /// 
    /// The `new` function will panic if the size is zero.
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);  // function will panic if not true

        let (sender, receiver) = mpsc::channel();
        let receiver 
                    = Arc::new(Mutex::new(receiver));
        let mut workers = Vec::with_capacity(size);

        // create threads
        for id in 0..size {
            /*
            /// This has an error on passing the receiver saying that we have already
            /// been moved the value in prev iteration. What we want is for our workers to have
            /// shared ownership of the receiver. In addition, listening for jobs requires mutating 
            /// the receiver so we want shared ownership and mutability. We can get this behavior by
            /// using smart pointers but because we are working with threads, we want thread-safe
            /// smart pointers. For thread-safe mutliple ownership, we can use the arc smart pointer and
            /// for thread-safe mutability, we can use the mutex smart pointer.
            */
            // workers.push(Worker::new(id, receiver);
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }

        ThreadPool { workers, sender }
    }

    pub fn execute<F>(&self, f: F) 
    where
        F: FnOnce() + Send + 'static
    {

    }
}


/*
/// Spawn function will take in a closure which will execute 
/// immediately after a thread is created. This is not what we want.
/// We want to create threads that wait for some code to execute later on. 
/// To get the behavior we want, instead of storing threads directly, we will
/// store another struct called worker. 
 */ 
struct Worker {
    id: usize,
    thread: thread::JoinHandle<()>
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        let thread = thread::spawn(||{
            receiver;
        });
        Worker { id, thread }
    } 
}