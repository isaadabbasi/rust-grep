use std::fs;
use std::error::Error;

pub struct Config {
  pub query: String,
  pub filename: String,
  pub i: bool,
}

impl Config {
  pub fn new(args: &[String]) -> Result<Config, &str> {

      if args.len() < 3 {
          return Err("Not enough arguments");
      }

      return Ok(Config {
          query:  args[1].clone(),
          filename: args[2].clone(),
          i: args[3].clone() == "i" || args[3].clone() == "-i",
      });
  }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
  let contents = fs::read_to_string(config.filename)?;

  let result = if config.i {
    searchi(&config.query, &contents)
  } else {
    search(&config.query, &contents)
  };

  for line in result {
    println!("{}", line);
  }

  Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
  let mut result: Vec<&str> = Vec::new();
  for line in contents.lines() {
    if line.contains(query) {
      result.push(line);
    }
  }
  return result;
}

pub fn searchi<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
  let mut result: Vec<&str> = Vec::new();
  let query = query.to_lowercase();
  for line in contents.lines() {
    if line.to_lowercase().contains(&query) {
      result.push(line);
    }
  }
  return result;
}
