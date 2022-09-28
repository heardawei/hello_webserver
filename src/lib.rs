use std::{fmt::Debug, thread};

pub struct ThreadPool {
  threads: Vec<thread::JoinHandle<()>>,
}

pub struct PoolCreationError;

impl ThreadPool {
  pub fn new(size: usize) -> Result<ThreadPool, PoolCreationError> {
    if size > 0 {
      let threads = Vec::with_capacity(size);
      for _ in 0..size {}
      Ok(ThreadPool { threads })
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

use std::fmt;

impl Debug for PoolCreationError {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    f.debug_tuple("").finish()
  }
}
