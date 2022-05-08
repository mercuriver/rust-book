use std::sync::mpsc;
use std::thread; // Multi-producer, single-consumer FIFO queue communication primitives.

fn main() {
  let (tx, rx) = mpsc::channel();

  thread::spawn(move || {
    let val = String::from("Hello");
    tx.send(val).unwrap();
  });

  let received = rx.recv().unwrap();
  println!("Received: {}", received);
}
