/*
 * 크리스마스 캐롤 “The Twelve Days of Christmas”의 가사를 반복문을 활용해 출력
*/

fn main() {
  let days = [
    "first", "second", "third", "fourth", "fifth", "sixth", "seventh", "eighth", "ninth", "tenth",
    "eleventh", "twelfth",
  ];

  let lyrics = [
    ["a partridge in a pear tree", "", "And "],
    ["Two turtle-doves", "", ""],
    ["Three French hens", "", ""],
    ["Four calling birds", "", ""],
    ["Five golden rings (five golden rings)", "", ""],
    ["Six geese a laying", "", ""],
    ["Seven swans a swimming", "", ""],
    ["Eight maids a milking", "", ""],
    ["Nine ladies dancing", "", ""],
    ["Ten lords a leaping", "", ""],
    ["Eleven pipers piping", "I sent ", ""],
    ["Twelve drummers drumming", "", ""],
  ];

  for (day_num, day) in days.iter().enumerate() {
    println!(
      "\n\nOn the {} day of Christmas\nMy true love sent to me",
      day
    );
    for i in (0..day_num + 1).rev() {
      if i == day_num {
        println!("{}{}", lyrics[i][1], lyrics[i][0]);
      } else {
        println!("{}{}", lyrics[i][2], lyrics[i][0]);
      }
    }
  }
}
