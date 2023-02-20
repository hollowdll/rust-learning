// Remove this for warnings about unused code
#![allow(unused)]

use std::thread;

pub struct ThreadPool {
    workers: Vec<Worker>,
}

impl ThreadPool {
    /// Create a new ThreadPool
    /// 
    /// The size is the number of threads in the pool.
    /// 
    /// # Panics
    /// 
    /// The `new` function will panic if the size is zero
    pub fn new(size: usize) -> Self {
        assert!(size > 0);
        
        let mut workers = Vec::with_capacity(size);

        for id in 0..size {
            // Create workers with empty threads
            workers.push(Worker::new(id));
        }

        Self {
            workers
        }
    }

    pub fn execute<F>(&self, _f: F)
    where
        F: FnOnce() + Send + 'static,
    {

    }
}

struct Worker {
    id: usize,
    thread: thread::JoinHandle<()>,
}

impl Worker {
    fn new(id: usize) -> Self {
        Self {
            id,
            thread: thread::spawn(|| {}),
        }
    }
}



#[cfg(test)]
mod tests {
    // TODO
    // Unit tests
}