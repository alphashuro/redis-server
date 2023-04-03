use std::{
    sync::{
        mpsc::{self, Receiver, Sender},
        Arc, Mutex,
    },
    thread,
};

type Job = Box<dyn FnOnce() + Send + 'static>;

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: Sender<Job>,
}

pub struct Worker {
    id: usize,
    thread: thread::JoinHandle<()>,
}
impl Worker {
    fn new(i: usize, receiver: Arc<Mutex<Receiver<Job>>>) -> Worker {
        let thread = thread::spawn(move || loop {
            let job = receiver.lock().unwrap().recv().unwrap();
            job();
        });

        Worker {
            id: i,
            thread: thread,
        }
    }
}

impl ThreadPool {
    /// Creates a new ThreadPool
    ///
    /// size is the number of threads required in the pool
    ///
    /// # Panics
    ///
    /// the `new` function will panic if the size is zero
    pub fn new(size: usize) -> Self {
        assert!(size > 0);

        let (sender, receiver) = mpsc::channel();
        let mut workers: Vec<Worker> = Vec::with_capacity(size);

        let receiver = Arc::new(Mutex::new(receiver));

        for i in 0..size {
            workers.push(Worker::new(i, Arc::clone(&receiver)));
        }

        ThreadPool { workers, sender }
    }

    pub fn execute<F>(&self, closure: F) -> ()
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(closure);

        self.sender.send(job);
    }
}
