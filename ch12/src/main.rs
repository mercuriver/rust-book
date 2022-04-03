use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = parse_config(&args);

    println!("검색어: {}", config.query);
    println!("대상 파일: {}", config.filename);

    let contents = fs::read_to_string(config.filename).expect("파일을 읽지 못했습니다.");

    println!("파일 내용: \n{}", contents);
}

struct Config {
    query: String,
    filename: String,
}

fn parse_config(args: &[String]) -> Config {
    let query = args[1].clone();
    let filename = args[2].clone();

    Config { query, filename }
}
