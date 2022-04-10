use std::thread;
use std::time::Duration;

fn simulated_expensive_caculation(intensity: u32) -> u32 {
  println!("시간이 오래 걸리는 작업.");
  thread::sleep(Duration::from_secs(2));
  intensity
}

fn generate_workout(intensity: u32, random_numbr: u32) {
  let expensive_closure = |num: u32| -> u32 {
    println!("시간이 오래 걸리는 작업.");
    thread::sleep(Duration::from_secs(2));
    num
  };

  if intensity < 25 {
    println!("팔 굽혀펴기 {}번", expensive_closure(intensity));
    println!("윗몸 일으키기 {}번", expensive_closure(intensity));
  } else {
    if random_numbr == 3 {
      println!("수분을 충분히 섭취하세요.");
    } else {
      println!("달리기 {}분", expensive_closure(intensity));
    }
  }
}

fn main() {
  let simulated_user_specified_value = 10;
  let simulated_random_number = 7;

  generate_workout(simulated_user_specified_value, simulated_random_number);
}
