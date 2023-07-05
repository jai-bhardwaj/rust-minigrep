use std::error::Error;
use std::fs;
use std::env;

pub fn run(Config {query, filename, case_sensitive}: Config) -> Result<(),Box<dyn Error>> {
    let contents = fs::read_to_string(filename)?;
    if case_sensitive {
        for line in search(&query, &contents) {
            println!("{}", line);
        }
    } else {
        for line in search_case_insensitive(&query, &contents) {
            println!("{}", line);
        }
    }

    Ok(())
}

pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
}

impl Config {
    pub fn new(mut args: env::Args) -> Result<Config,&'static str> {
        args.next();
        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a query string"),
        };
        let filename = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a file name"),
        };

        let case_sensitive = env::var("CASE_SENSITIVE").is_err();

        Ok(Config {query, filename, case_sensitive})
    }
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|x| x.contains(&query))
        .collect()
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results: Vec<&'a str> = Vec::new();
    let query = query.to_lowercase();
    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line.trim());
        }
    }
    results
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "\
        Rust:
        safe, fast, productive.
        Pick Three.";
        assert_eq!(vec!["safe, fast, productive."],search(query,contents));
    }

    #[test]
    fn case_insensitive() {
        let query = "ruST";
        let contents = "\
        Rust:
        safe, fast, productive.
        Pick Three.
        Trust me.";
        assert_eq!(vec!["Rust:","Trust me."],search_case_insensitive(query,contents));
    }
}