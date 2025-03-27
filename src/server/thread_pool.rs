use std::{sync::{mpsc, Arc, Mutex}, thread::{self, JoinHandle}};

type Job = Box<dyn FnOnce() + Send + 'static>;

pub struct ThreadPool {
    pool_size: usize,
    threads: Vec<Worker>,
    sender: mpsc::Sender<Job>
}

impl ThreadPool {
    pub fn new(pool_size: usize) -> ThreadPool {
        assert!(pool_size > 0);
        let mut threads = Vec::with_capacity(pool_size);
        let (tx, rx) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(rx));
    
        for id in 0..pool_size {
            threads.push(Worker::new(id, Arc::clone(&receiver)));
        }

        ThreadPool { pool_size: pool_size, threads: threads, sender:tx }
    }

    pub fn execute<F>(&self, f: F)
        where F: FnOnce() + Send + 'static
        {
            let job = Box::new(f);

            self.sender.send(job).unwrap();
        }
}

struct Worker {
    id: usize,
    handle: JoinHandle<()>,
}

impl Worker {
    fn new(worker_id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        let thread = thread::spawn(move || loop {
            let job = receiver.lock().unwrap().recv().unwrap();

            job();
        });
        Worker { id: worker_id, handle: thread }
    }
}