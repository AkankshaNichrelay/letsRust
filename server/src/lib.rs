// in order to make our library crate our primary crate, we move main.rs into bin folder.

pub struct ThreadPool;

impl ThreadPool {
    pub fn new(size: usize) -> ThreadPool {
        // not using size yet; Compilier driven development
        ThreadPool
    }

    pub fn execute<F>(&self, f: F) 
    where
        F: FnOnce() + Send + 'static
    {

    }
}