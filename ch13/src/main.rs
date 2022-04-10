use std::thread;
use std::time::Duration;

struct Cacher<T>
where
  T: Fn(u32) -> u32,
{
  calculation: T,
  value: Option<u32>,
}

impl<T> Cacher<T>
where
  T: Fn(u32) -> u32,
{
  fn new(calculation: T) -> Cacher<T> {
    Cacher {
      calculation,
      value: None,
    }
  }
  fn value(&mut self, arg: u32) -> u32 {
    match self.value {
      Some(v) => v,
      None => {
        let v = (self.calculation)(arg);
        self.value = Some(v);
        v
      }
    }
  }
}

fn generate_workout(intensity: u32, random_numbr: u32) {
  let mut expensive_result = Cacher::new(|num: u32| -> u32 {
    println!("시간이 오래 걸리는 작업.");
    thread::sleep(Duration::from_secs(2));
    num
  });

  if intensity < 25 {
    println!("팔 굽혀펴기 {}번", expensive_result.value(intensity));
    println!("윗몸 일으키기 {}번", expensive_result.value(intensity));
  } else {
    if random_numbr == 3 {
      println!("수분을 충분히 섭취하세요.");
    } else {
      println!("달리기 {}분", expensive_result.value(intensity));
    }
  }
}

fn main() {
  let simulated_user_specified_value = 10;
  let simulated_random_number = 7;

  generate_workout(simulated_user_specified_value, simulated_random_number);
}
