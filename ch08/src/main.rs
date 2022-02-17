/*
 * 해쉬맵과 벡터를 이용하여, 사용자가 회사 내의 부서에 대한 피고용인 이름을
 * 추가할 수 있도록 하는 텍스트 인터페이스를 만들어보세요.
 * 예를들어 “Add Sally to Engineering”이나 “Add Amir to Sales” 같은 식으로요.
 * 그후 사용자가 각 부서의 모든 사람들에 대한 리스트나 알파벳 순으로 정렬된
 * 부서별 모든 사람에 대한 리스트를 조회할 수 있도록 해보세요.
*/
use std::collections::HashMap;
use std::io;

fn trim_input(value: &String) -> &str {
  value.trim()
}

fn print_type_of<T>(_: &T) {
  println!("{}", std::any::type_name::<T>());
}

// #[derive(Debug)]
// struct Employee {
//   name: String,
//   depertment: String,
// }

// impl Employee {
//   fn new(name: &str, depertment: &str) -> Employee {
//     Employee { name: name.to_string(), depertment: depertment.to_string() }
//   }
// }

fn parse_command(command: &str, data: &mut HashMap<String, String>) {
  match command {
    "ADD" => {
      println!("사원 정보 입력");
      println!("ex) \"Amir to Sales\"");

      let mut detail = String::new();
      io::stdin()
        .read_line(&mut detail)
        .expect("유효하지 않은 입력입니다.");

      let keywords: Vec<&str> = trim_input(&detail).split(' ').collect();
      // println!("{:?}", keywords);
      // print_type_of(&keywords);
      // print_type_of(&keywords[0]);
      // println!("{}", keywords[0]);

      // &str(문자열 슬라이스) 보다 문자열로 다루면 되기에 형 변환
      // keywords로 접근할 경우 &detail의 라이프타임 문제가 발생
      let name: String = keywords[0].to_string();
      let depertment: String = keywords[2].to_string();

      data.insert(name, depertment);
    }
    "REMOVE" => println!("사원 삭제 절차 진행"),
    "EDIT" => println!("사원 삭제 절차 진행"),
    "SEARCH" => println!("사원 삭제 절차 진행 [Name|Depertment]"),
    "ORDER" => println!("정렬 진행 [Asc|Desc]"),
    "PRINT" => {
      println!("\n\n{:?}", data);
    }
    _ => println!("유효하지 않은 작업입니다."),
  }
}

fn main() {
  let mut data: HashMap<String, String> = HashMap::new();
  // let mut data = HashMap::new();

  loop {
    println!("작업 형식 입력");
    println!("[Add, Remove, Edit, Search, Order, Print, Exit]");

    let mut command = String::new();

    io::stdin()
      .read_line(&mut command)
      .expect("유효하지 않은 입력입니다.");
    command.make_ascii_uppercase();

    let command: &str = trim_input(&command);

    println!("{}", command);

    match command {
      "EXIT" => {
        println!("프로그램 종료");
        break;
      }
      _ => parse_command(&command, &mut data),
    }
  }
}
