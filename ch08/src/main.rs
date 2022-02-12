/*
 * 해쉬맵과 벡터를 이용하여, 사용자가 회사 내의 부서에 대한 피고용인 이름을 
 * 추가할 수 있도록 하는 텍스트 인터페이스를 만들어보세요. 
 * 예를들어 “Add Sally to Engineering”이나 “Add Amir to Sales” 같은 식으로요. 
 * 그후 사용자가 각 부서의 모든 사람들에 대한 리스트나 알파벳 순으로 정렬된 
 * 부서별 모든 사람에 대한 리스트를 조회할 수 있도록 해보세요.
*/
use std::io;

// fn checkInput(value, typeCase) {
//   let value: u32 = match value.trim().parse() {
//     Ok(num) => num,
//     Err(_) => continue,
//   };
//   value
// }

fn parse_command(command: &str) {
  match command {
    "ADD" => println!("사원 등록 절차 진행"),
    "REMOVE" => println!("사원 삭제 절차 진행"),
    "EDIT" => println!("사원 삭제 절차 진행"),
    "SEARCH" => println!("사원 삭제 절차 진행 [Name|Depertment]"),
    "ORDER" => println!("정렬 진행 [Asc|Desc]"),
    "PRINT" => println!("전체 출력"),
    _ => println!("유효하지 않은 작업입니다."),
  }
}

fn main() {
  loop {
    println!("작업 형식 입력");
    println!("[Add, Remove, Edit, Search, Order, Print, Exit]");

    let mut command = String::new();

    io::stdin().read_line(&mut command)
        .expect("유효하지 않은 입력입니다.");
    command.make_ascii_uppercase();

    let command: &str = command.trim();

    println!("{}", command);

    match command {
      "EXIT" => {
          println!("프로그램 종료");
          break;
      }
      _ => parse_command(&command),
    }
  }
}
