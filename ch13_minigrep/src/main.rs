use std::env;
use std::process;

use ch13_minigrep::Config;

fn main() {
    let config = Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("인수를 구문분석하는 동안 오류가 발생했습니다: {}", err);
        process::exit(1);
    });

    if let Err(e) = ch13_minigrep::run(config) {
        eprintln!("애플리케이션 에러: {}", e);
        process::exit(1);
    }
}
