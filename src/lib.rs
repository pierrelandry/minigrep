use std::error::Error;
use std::{env, fs};

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let file_contents = fs::read_to_string(config.file_path)?;

    let results = if config.ignore_case {
        search(&config.query, &file_contents)
    } else {
        search_case_insensitive(&config.query, &file_contents)
    };

    for line in results {
        println!("{line}")
    }

    Ok(())
}

pub struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
}

impl Config {
    pub fn build(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
        args.next(); //ignore the first value which is the name of the program

        let query = match args.next() {
            Some(arg) =>arg,
            None => return Err("Please provide a query"),
        };

        let file_path = match args.next() {
            Some(arg) => arg,
            None => return Err("Please provide a file path"),
        };

        let ignore_case = env::var("IGNORE_CASE").is_ok(); //set or not set

        Ok(Config {
            query,
            file_path,
            ignore_case,
        })
    }
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents.lines().filter(|line| line.contains(query)).collect()
}


pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    contents.lines().filter(|line| line.to_lowercase().contains(&query)).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn search_should_return_one_line_containing_string() {
        // given
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        // when
        let result = search(query, contents);

        assert_eq!(vec!["safe, fast, productive."], result);
    }

    #[test]
    fn search_should_be_case_sensitive() {
        // given
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Duct three.";

        // when
        let result = search(query, contents);

        // then
        assert_eq!(vec!["safe, fast, productive."], result);
    }

    #[test]
    fn search_case_insensitive_should_return_two_lines() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, contents)
        );
    }
}