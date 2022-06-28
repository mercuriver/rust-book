use std::thread;

pub struct ThreadPool {
  threads: Vec<thread::JoinHandle<()>>,
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

    let mut threads = Vec::with_capacity(size);

    for _ in 0..size {}

    ThreadPool { threads }
  }

  pub fn execute<F>(&self, f: F)
  where
    F: FnOnce() + Send + 'static,
  {
  }
}
