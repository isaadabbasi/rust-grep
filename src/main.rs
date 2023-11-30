use std::env;
use std::process;

use minigrep::Config;
use minigrep::run;

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("Hello, {:?}!", args);

    let config: Config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    println!("Searching for \"{}\" in file \"{}\" \n", config.query, config.filename);

    if let Err(e) = run(config) {
        println!("Application error: {}", e);
        process::exit(1);
    }
}

// add test
#[cfg(test)]
mod tests {
    use minigrep::search;
    use minigrep::searchi;


    #[test]
    fn case_sensetive_match() {
        let query = "Rust";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

    assert_eq!(vec!["Rust:"], search(query, contents));
    }

    #[test]
    fn case_insensetive_match() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

    assert_eq!(vec!["Rust:"], searchi(query, contents));
    }
}
