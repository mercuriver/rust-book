use std::sync::mpsc;
use std::thread;

struct Job;

struct Worker {
  id: usize,
  thread: thread::JoinHandle<()>,
}

impl Worker {
  fn new(id: usize, receiver: mpsc::Receiver<Job>) -> Worker {
    let thread = thread::spawn(|| {
      receiver;
    });
    Worker { id, thread }
  }
}

pub struct ThreadPool {
  workers: Vec<Worker>,
  sender: mpsc::Sender<Job>,
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

    let (sender, receiver) = mpsc::channel();

    let mut workers = Vec::with_capacity(size);

    for id in 0..size {
      workers.push(Worker::new(id, receiver))
    }

    ThreadPool { workers, sender }
  }

  pub fn execute<F>(&self, f: F)
  where
    F: FnOnce() + Send + 'static,
  {
  }
}
