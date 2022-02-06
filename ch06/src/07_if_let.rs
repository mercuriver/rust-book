#[derive(Debug)]
enum UsState {
  Alabama,
  Alaska, 
  Etc,
}

enum Coin {
  Penny,
  Nickle,
  Dime,
  Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u32 {
  let mut count: u32 = 0;
  match coin {
    Coin::Quarter(state) => println!("{:?}주의 25센트 코인", state),
    _ => count += 1,
  }
  count
}


fn let_if_value_in_cents(coin: Coin) -> u32 {
  let mut count: u32 = 0;
  if let Coin::Quarter(state) = coin {
    println!("{:?}주의 25센트 코인", state);
  } else {
    count += 1;
  }
  count
}

fn main() {
  let penny = value_in_cents(Coin::Penny);
  let nickle = value_in_cents(Coin::Nickle);
  let dime = value_in_cents(Coin::Dime);
  let quarter = value_in_cents(Coin::Quarter(UsState::Alabama));
  let quarter2 = value_in_cents(Coin::Quarter(UsState::Alaska));
  let dime2 = value_in_cents(Coin::Dime);
  let nickle2 = value_in_cents(Coin::Nickle);
  let quarter3 = value_in_cents(Coin::Quarter(UsState::Etc));
  let dime3 = value_in_cents(Coin::Dime);

  println!("Coin list: {}, {}, {}, {}", penny, nickle, dime, quarter);
  println!("Coin list: {}, {}, {}, {}, {}", quarter2, dime2, nickle2, quarter3, dime3);


  let _quarter = let_if_value_in_cents(Coin::Quarter(UsState::Alabama));
  let _nickle = let_if_value_in_cents(Coin::Nickle);
  println!("## Coin list: {}, {}", _quarter, _nickle);
}
