/*
 * 화씨와 섭씨를 상호 변환
*/
use std::io;

fn f_to_c_converter(value: f32) -> f32 {
  (value - 32.0) * (5.0 / 9.0)
}

fn c_to_f_converter(value: f32) -> f32 {
  (value * (9.0 / 5.0)) + 32.0
}

fn main() {
  let mut temperature = String::new();

  io::stdin()
    .read_line(&mut temperature)
    .expect("입력한 값을 읽지 못했습니다.");

  let temperature: f32 = temperature.trim().parse::<f32>().unwrap();

  println!(
    "Input:{}, Fahrenheit: {}°F, Celsius: {}°C",
    temperature,
    f_to_c_converter(temperature),
    c_to_f_converter(temperature)
  );
}
