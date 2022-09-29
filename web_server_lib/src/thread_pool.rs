use std::error::Error;
use std::sync::mpsc::{Receiver, Sender};
use std::sync::{mpsc, Arc, Mutex};
use std::thread;
use std::thread::JoinHandle;

type Job = Box<dyn FnOnce() + Send + 'static>;

#[derive(Debug)]
struct ThreadWorker {
    id: String,
    handle: Option<JoinHandle<()>>,
}

impl ThreadWorker {
    fn new(idx: usize, receiver: Arc<Mutex<Receiver<Job>>>) -> ThreadWorker {
        ThreadWorker {
            id: format!("Worker_{}", idx),
            handle: Some(thread::spawn(move || loop {
                let message = receiver.lock().unwrap().recv();
                if let Ok(job) = message {
                    job();
                } else {
                    println!("Worker Worker_{idx} disconnected; Shutting down");
                    break;
                }
            })),
        }
    }
}

#[derive(Debug)]
pub struct WorkerPool {
    sender: Option<Sender<Job>>,
    threads: Vec<ThreadWorker>,
}

impl WorkerPool {
    pub fn execute<F>(&self, f: F) -> Result<(), Box<dyn Error>>
    where
        F: FnOnce() + Send + 'static,
    {
        let job: Job = Box::new(f);
        if let Some(sender) = &self.sender {
            sender.send(job)?;
        }
        Ok(())
    }

    pub fn new(size: usize) -> WorkerPool {
        assert!(size > 0);

        let mut threads = Vec::with_capacity(size);

        let (sender, receiver) = mpsc::channel();

        let receiver = Arc::new(Mutex::new(receiver));

        for i in 0..size {
            threads.insert(i, ThreadWorker::new(i, Arc::clone(&receiver)));
        }
        WorkerPool {
            sender: Some(sender),
            threads,
        }
    }
}

impl Drop for WorkerPool {
    fn drop(&mut self) {
        drop(self.sender.take());
        for worker in &mut self.threads {
            println!("Shutting down worker {}", worker.id);
            if let Some(thread) = worker.handle.take() {
                thread.join().unwrap();
            }
        }
    }
}
