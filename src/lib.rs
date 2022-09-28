pub struct ThreadPool;
pub struct PoolCreationError;

// use std::thread;

impl ThreadPool {
  pub fn new(size: usize) -> Result<ThreadPool, PoolCreationError> {
    if size > 0 {
      Ok(ThreadPool)
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
