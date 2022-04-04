use std::error::Error;
use std::fs;

pub struct Config {
  pub query: String,
  pub filename: String,
}

impl Config {
  pub fn new(args: &[String]) -> Result<Config, &'static str> {
    if args.len() < 3 {
      return Err("필요한 인수가 지정되지 않았습니다.");
    }

    let query = args[1].clone();
    let filename = args[2].clone();
    Ok(Config { query, filename })
  }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
  let contents = fs::read_to_string(config.filename)?;
  println!("파일 내용: \n{}", contents);

  Ok(())
}

// impl Config {
//   fn new(args: &[String]) -> Config {
//     let query = args[1].clone();
//     let filename = args[2].clone();
//     Config { query, filename }
//   }
// }
