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
  match coin {
    Coin::Penny => {
      println!("Penny Penny!");
      1
    },
    Coin::Nickle => 5,
    Coin::Dime => 10,
    Coin::Quarter(state) => {
      println!("State quarter from {:?}", state);
      25
    },
  }
}

fn main() {
  let penny = value_in_cents(Coin::Penny);
  let nickle = value_in_cents(Coin::Nickle);
  let dime = value_in_cents(Coin::Dime);
  let quarter = value_in_cents(Coin::Quarter(UsState::Alabama));

  println!("Coin list: {}, {}, {}, {}", penny, nickle, dime, quarter);
}
