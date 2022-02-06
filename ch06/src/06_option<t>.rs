fn plus_one(x: Option<i32>) -> Option<i32> {
  match x {
    None => None,
    Some(i) => Some(i + 1),
  }
}

fn check_value(x: u8) {
  match x {
    1 => println!("one"),
    3 => println!("three"),
    5 => println!("five"),
    7 => println!("seven"),
    _ => (),
  }  
}

fn main() {
  let five = Some(5);
  let six = plus_one(five);
  let none = plus_one(None);

  println!("Result: {:?}, {:?}, {:?}", five, six, none);

  println!("### match default");
  check_value(0u8);
  check_value(1);
  check_value(2);
  check_value(3);

}


