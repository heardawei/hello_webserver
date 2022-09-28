use std::{fmt::Debug, thread};

pub struct ThreadPool {
  workers: Vec<Worker>,
}

struct Worker {
  id: usize,
  thread: thread::JoinHandle<()>,
}

impl ThreadPool {
  pub fn new(size: usize) -> Result<ThreadPool, PoolCreationError> {
    if size > 0 {
      let mut workers = Vec::with_capacity(size);
      for index in 0..size {
        workers.push(Worker::new(index));
      }
      Ok(ThreadPool { workers })
    } else {
      Err(PoolCreationError)
    }
  }
  pub fn execute<F>(&self, f: F)
  where
    F: FnOnce(),
    F: Send + 'static,
  {
  }
  // pub fn execute<F, T>(&self, f: F) -> thread::JoinHandle<T>
  // where
  //   F: FnOnce() -> T,
  //   F: Send + 'static,
  //   T: Send + 'static,
  // {
  // }
}

impl Worker {
  fn new(id: usize) -> Worker {
    let thread = thread::spawn(|| {});
    Worker { id, thread }
  }
}

use std::fmt;

pub struct PoolCreationError;

impl Debug for PoolCreationError {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    f.debug_tuple("").finish()
  }
}
