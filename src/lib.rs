use std::{
  fmt,
  sync::{mpsc, Arc, Mutex},
  thread,
};

pub struct ThreadPool {
  workers: Vec<Worker>,
  sender: mpsc::Sender<Job>,
}

struct Worker {
  id: usize,
  thread: thread::JoinHandle<()>,
}

type Job = Box<dyn FnOnce() + Send + 'static>;

impl ThreadPool {
  pub fn new(size: usize) -> Result<ThreadPool, PoolCreationError> {
    if size > 0 {
      let (sender, receiver) = mpsc::channel();
      let receiver = Arc::new(Mutex::new(receiver));
      let mut workers = Vec::with_capacity(size);
      for index in 0..size {
        workers.push(Worker::new(index, receiver.clone()));
      }
      Ok(ThreadPool { workers, sender })
    } else {
      Err(PoolCreationError)
    }
  }
  pub fn execute<F>(&self, f: F)
  where
    F: FnOnce() + Send + 'static,
  {
    let job = Box::new(f);
    self.sender.send(job).unwrap();
  }
}

impl Worker {
  fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
    let thread = thread::spawn(move || loop {
      let job = receiver.lock().unwrap().recv().unwrap();
      println!("Worker {id}: got a job; executing..");
      job();
      println!("Worker {id}: got a job; executed...");
    });
    Worker { id, thread }
  }
}

pub struct PoolCreationError;

impl fmt::Debug for PoolCreationError {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    f.debug_tuple("").finish()
  }
}
