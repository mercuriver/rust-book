/*
 * 스트링을 피그 라틴(pig Latin)으로 변경해보세요. 
 각 단어의 첫번째 자음은 단어의 끝으로 이동하고 “ay”를 붙이므로, “first”는 “irst-fay”가 됩니다. 
 모음으로 시작하는 단어는 대신 끝에 “hay”를 붙입니다. 
 (“apple”은 “apple-hay”가 됩니다.) 
 UTF-8 인코딩에 대해 기억하세요!
*/
use std::io;

fn main() {
	println!("단어 입력: ");

	let mut keyword = String::new();
	io::stdin().read_line(&mut keyword)
		.expect("Failed to read line");

	keyword.pop(); // Tear off carriage returns
	let rest = keyword.split_off(1);
	let first_char = keyword;
	
	if ['a', 'e', 'i', 'o', 'u'].contains(&first_char.chars().next().unwrap()) {
		println!("{}{}-hay", first_char, rest);
	} else {
		println!("{}-{}ay", rest, first_char);
	}
}
