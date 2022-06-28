use std::thread;

struct Worker {
  id: usize,
  thread: thread::JoinHandle<()>,
}

impl Worker {
  fn new(id: usize) -> Worker {
    let thread = thread::spawn(|| {});
    Worker { id, thread }
  }
}

pub struct ThreadPool {
  workers: Vec<Worker>,
}

impl ThreadPool {
  /// 새 ThreadPool 인스턴스 생성
  ///  
  /// size 매개변수로 Thread pool size 지정
  ///
  /// # Panics
  ///
  /// size 매개변수의 값은 '0' 이상이여야 함
  pub fn new(size: usize) -> ThreadPool {
    assert!(size > 0);

    let mut workers = Vec::with_capacity(size);

    for id in 0..size {
      workers.push(Worker::new(id))
    }

    ThreadPool { workers }
  }

  pub fn execute<F>(&self, f: F)
  where
    F: FnOnce() + Send + 'static,
  {
  }
}
