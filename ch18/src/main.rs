enum Message {
    Hello { id: i32 },
}

fn find_id(msg: Message) {
    match msg {
        Message::Hello {
            id: id_variable @ 3..=7,
        } => {
            println!("id를 범위에서 찾았습니다: {}", id_variable)
        }
        Message::Hello { id: 10..=12 } => {
            println!("id를 다른 범위에서 찾았습니다.")
        }
        Message::Hello { id } => {
            println!("다른 id를 찾았습니다. {}", id)
        }
        _ => println!("나머지"),
    }
}

fn main() {
    find_id(Message::Hello { id: 5 });
    find_id(Message::Hello { id: 7 });
    find_id(Message::Hello { id: 12 });
    find_id(Message::Hello { id: 17 });
    // find_id(None);
}
// fn main() {
//     let x = Some(5);
//     let y = 10;

//     match x {
//         Some(50) => println!("Got 50"),
//         Some(n) if n == y => println!("일치, y = {:?}", y),
//         _ => println!("일치하지 않음, x = {:?}", x),
//     }

//     println!("결과: x = {:?}, y = {:?}", x, y);
// }
// fn main() {
//     let num = Some(4);

//     match num {
//         Some(x) if x < 5 => println!("5보다 작은 값: {}", x),
//         Some(x) => println!("{}", x),
//         None => (),
//     }
// }

// fn main() {
//     // let x = Some(5);
//     let x = None;
//     let y = 10;
//     match x {
//         Some(50) => println!("50"),
//         Some(y) => println!("일치, y = {:?}", y),
//         _ => println!("일치하지 않음, x = {:?}", x),
//     }
//     println!("결과: x = {:?}, y = {:?}", x, y);
// }

// fn main() {
//     let s1 = Some(String::from("Hello 1!"));
//     let s2 = Some(String::from("Hello 2!"));

//     if let Some(_s) = s1 {
//         println!("문자열을 찾았습니다 1.");
//     }

//     if let Some(_) = s2 {
//         println!("문자열을 찾았습니다 2.");
//     }

//     println!("{:?}, {:?}", s1, s2);
// }

// fn main() {
//     let mut setting_value = Some(5);
//     let new_setting_value = Some(10);

//     match (setting_value, new_setting_value) {
//         (Some(_), Some(_)) => {
//             println!("이미 설정된 값을 덮어쓸 수 없습니다.");
//         }
//         _ => {
//             setting_value = new_setting_value;
//         }
//     }

//     println!("현재 설정 {:?}", setting_value);
// }

// struct Point {
//     x: i32,
//     y: i32,
// }

// fn main() {
//     let points = vec![
//         Point { x: 0, y: 0 },
//         Point { x: 1, y: 5 },
//         Point { x: 10, y: -3 },
//     ];

//     let sum_of_squares: i32 = points.iter().map(|Point { x, y }| x * x + y * y).sum();
//     println!("{}", sum_of_squares);
// }

// enum Color {
//     Rgb(i32, i32, i32),
//     Hsv(i32, i32, i32),
// }

// enum Message {
//     Quit,
//     Move { x: i32, y: i32 },
//     Write(String),
//     ChangeColor(Color),
// }

// fn parse_message(msg: Message) {
//     match msg {
//         Message::ChangeColor(Color::Rgb(r, g, b)) => {
//             println!("ChangeColor: R = {}, G = {}, B = {}", r, g, b)
//         }
//         Message::ChangeColor(Color::Hsv(h, s, v)) => {
//             println!("ChangeColor: H = {}, S = {}, V = {}", h, s, v)
//         }
//         _ => {}
//     }
// }

// fn main() {
//     parse_message(Message::ChangeColor(Color::Rgb(100, 100, 100)));
//     parse_message(Message::ChangeColor(Color::Hsv(0, 160, 255)));
// }

// enum Message {
//     Quit,
//     Move { x: i32, y: i32 },
//     Write(String),
//     ChangeColor(i32, i32, i32),
// }

// fn parse_message(msg: Message) {
//     match msg {
//         Message::Quit => {
//             println!("Quit: 해체할 값이 없습니다.")
//         }
//         Message::Move { x, y } => {
//             println!("Move: x = {}, y = {}", x, y);
//         }
//         Message::Write(text) => println!("Write: {}", text),
//         Message::ChangeColor(r, g, b) => {
//             println!("ChangeColor: R = {}, G = {}, B = {}", r, g, b)
//         }
//     }
// }

// fn main() {
//     parse_message(Message::Quit);
//     parse_message(Message::Move { x: 1, y: 2 });
//     parse_message(Message::Write("쓰뜨링".to_string()));
//     parse_message(Message::ChangeColor(0, 160, 255));
// }

// struct Point {
//     x: i32,
//     y: i32,
// }

// fn parse_point(p: Point) {
//     match p {
//         Point { x, y: 0 } => println!("x 축 {}에 위치하는 점", x),
//         Point { x: 0, y } => println!("y 축 {}에 위치하는 점", y),
//         Point { x, y } => println!("좌표: ({}, {})", x, y),
//     }
// }

// fn main() {
//     parse_point(Point { x: 0, y: 1 });
//     parse_point(Point { x: 2, y: 0 });
//     parse_point(Point { x: 3, y: 3 })
// }

// struct Qoint {
//     a: i32,
//     b: i32,
// }

// fn main() {
//     let q = Qoint { a: 0, b: 7 };
//     let p = Point { x: 0, y: 7 };

//     let Qoint { a: a, b: b } = q;
//     assert_eq!(0, a);
//     assert_eq!(7, b);

//     let Point { x, y } = p;
//     assert_eq!(0, x);
//     assert_eq!(7, y);
// }

// fn main() {
//     let x = 'c';

//     match x {
//         'a'..='j' => println!("ASCII 소문자 전반부"),
//         'k'..='z' => println!("ASCII 소문자 후반부"),
//         _ => println!("소문자 나머지"),
//     }
//     match x {
//         'A'..='J' => println!("ASCII 대문자 전반부"),
//         'K'..='Z' => println!("ASCII 대문자 후반부"),
//         _ => println!("대문자 나머지"),
//     }
// }

// fn main() {
//     // let x = Some(5);
//     let x = None;
//     let y = 10;
//     match x {
//         Some(50) => println!("50"),
//         Some(y) => println!("일치, y = {:?}", y),
//         _ => println!("일치하지 않음, x = {:?}", x),
//     }
//     println!("결과: x = {:?}, y = {:?}", x, y);
// }

// fn main() {
//     if let x = 5 {
//         println!("{}", x);
//     };
// }

// fn main() {
//     let some_option_value = Some(5);
//     let Some(x) = some_option_value;
// }

// fn main() {
//     let v = vec!['a', 'b', 'c'];

//     for (index, value) in v.iter().enumerate() {
//         println!("인덱스 {}의 값: {}", value, index);
//     }
// }

// fn main() {
//     let mut stack = Vec::new();

//     stack.push(1);
//     stack.push(2);
//     stack.push(3);

//     while let Some(top) = stack.pop() {
//         println!("{}", top);
//     }
// }

// fn main() {
//     let favorite_color: Option<&str> = None;
//     // let favorite_color: Option<&str> = Some("노랑");
//     let is_tuesday = false;
//     let age: Result<u8, _> = "34".parse();

//     if let Some(color) = favorite_color {
//         println!("선호하는 {}색을 배경으로 사용", color);
//     } else if is_tuesday {
//         println!("화요일엔 녹색!");
//     } else if let Ok(age) = age {
//         if age > 30 {
//             println!("보라색을 배경으로 사용");
//         } else {
//             println!("오랜지색을 배경으로 사용");
//         }
//     } else {
//         println!("파란색을 배경으로 사용");
//     }

//     println!("값 체크 {:?}", age);
// }
