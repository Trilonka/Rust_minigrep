use std::fs;
use std::error::Error;

pub struct Config {
    pub query: String,
    pub filename: String,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Not enough arguments");
        }

        if args.len() > 3 {
            return Err("Too much arguments");
        }
        
        let query = args[1].clone();
        let filename = args[2].clone();

        Ok(Config { query, filename })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;

    for line in search(&config.query, &contents) {
        println!("{}", line);
    }

    Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }

    results
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic]
    fn use_not_enough() {
        let v = vec![String::from("program"), String::from("hello")];
        if let Err(_) = Config::new(&v) {
            panic!();
        }
    }

    #[test]
    #[should_panic]
    fn use_too_much() {
        let v = vec![String::from("program"), String::from("hello"), String::from("hello"), String::from("hello")];
        if let Err(_) = Config::new(&v) {
            panic!();
        }
    }

    #[test]
    fn use_ok() {
        let v = vec![String::from("program"), String::from("hello"), String::from("poem.txt")];
        if let Err(_) = Config::new(&v) {
            panic!();
        }
    }

    #[test]
    #[should_panic]
    fn run_not_ok() {
        let v = vec![String::from("program"), String::from("hello"), String::from("pm.txt")];
        if let Err(_) = run(Config::new(&v).unwrap()) {
            panic!();
        }
    }

    #[test]
    fn run_ok() {
        let v = vec![String::from("program"), String::from("hello"), String::from("poem.txt")];
        if let Err(_) = run(Config::new(&v).unwrap()) {
            panic!();
        }
    }

    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "\nRust:
        \nsafe, fast, productive.
        \nPick three.";
        assert_eq!(
            vec!["safe, fast, productive."],
            search(query, contents)
        );
    }
}