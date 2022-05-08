use std::thread;
// use std::time::Duration;

fn main() {
  let v = vec![1, 2, 3];
  let handle = thread::spawn(move || {
    println!("Vector: {:?}", v);
  });

  handle.join().unwrap(); // case 1
}

// fn main() {
//   let handle = thread::spawn(|| {
//     for i in 1..10 {
//       println!("New thread: {}", i);
//       thread::sleep(Duration::from_millis(1));
//     }
//   });

//   // handle.join().unwrap(); // case 2

//   for i in 1..5 {
//     println!("Main thread: {}", i);
//     thread::sleep(Duration::from_millis(1));
//   }

//   handle.join().unwrap(); // case 1
// }

// fn main() {
//   thread::spawn(|| {
//     for i in 1..10 {
//       println!("New thread: {}", i);
//       thread::sleep(Duration::from_millis(1));
//     }
//   });

//   for i in 1..5 {
//     println!("Main thread: {}", i);
//     thread::sleep(Duration::from_millis(1));
//   }
// }
