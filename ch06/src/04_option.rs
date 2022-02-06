fn main() {
  let some_number = Some(5);
  let some_string = Some("a string");

  let absent_number: Option<i32> = None;

  println!("some_number: {:?}", some_number);
  println!("some_string: {:?}", some_string);
  println!("absent_number: {:?}", absent_number);

  // let x: i8 = 5;
  // let y: Option<i8> = Some(5);
  // let sum = x + y;
  // println!("x: {}", x);
  // println!("y: {}", y);
  // println!("sum: {}", sum);
}
