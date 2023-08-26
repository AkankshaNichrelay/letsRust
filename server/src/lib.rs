// in order to make our library crate our primary crate, we move main.rs into bin folder.

use std::thread;

pub struct ThreadPool {
    threads: Vec<thread::JoinHandle<()>>,
}

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

        let threads = Vec::with_capacity(size);

        // populate the threads
        for _ in 0..size {
            // create threads
        }

        ThreadPool { threads }
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
    fn new(id: usize) -> Worker {
        let thread = thread::spawn(||{});
        Worker { id, thread }
    } 
}