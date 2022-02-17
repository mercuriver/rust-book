/*
 * n번째 피보나치 수열 생성
*/

fn main() {
  let range = 20;
  let mut list = Vec::new();
  let mut prev_value = 0;
  let mut current_value = 0;

  for i in 0..range {
    let temp = if i == 0 { 1 } else { current_value };
    current_value = current_value + prev_value;
    prev_value = temp;
    list.push(current_value);
  }

  println!("{:?}", list);
}
