use std::{
  fmt,
  sync::{mpsc, Arc, Mutex},
  thread,
};

pub struct ThreadPool {
  workers: Vec<Worker>,
  sender: mpsc::Sender<Message>,
}

struct Worker {
  id: usize,
  thread: Option<thread::JoinHandle<()>>,
}

type Job = Box<dyn FnOnce() + Send + 'static>;
enum Message {
  NewJob(Job),
  Terminate,
}

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
    self.sender.send(Message::NewJob(job)).unwrap();
  }
}

impl Drop for ThreadPool {
  fn drop(&mut self) {
    for _ in &self.workers {
      self.sender.send(Message::Terminate).unwrap();
    }

    for worker in &mut self.workers {
      println!("Shutting down worker {}", worker.id);
      if let Some(thread) = worker.thread.take() {
        thread.join().unwrap();
      }
    }
  }
}

impl Worker {
  fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Message>>>) -> Worker {
    let thread = thread::spawn(move || loop {
      match receiver.lock().unwrap().recv().unwrap() {
        Message::NewJob(job) => {
          println!("Worker {id}: got a job; executing..");
          job();
          println!("Worker {id}: got a job; executed...");
        }
        Message::Terminate => {
          println!("Worker {} was told to terminate.", id);
          break;
        }
      }
    });
    Worker {
      id,
      thread: Some(thread),
    }
  }
}

pub struct PoolCreationError;

impl fmt::Debug for PoolCreationError {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    f.debug_tuple("").finish()
  }
}
