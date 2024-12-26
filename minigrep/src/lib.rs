use std::env;
use std::error::Error;
use std::fs;

#[derive(Debug)]
pub struct Config {
    pub needle: String,
    pub haystack: String,
    pub case_sensitive: bool,
}

impl Config {
    pub fn new(mut args: std::env::Args) -> Result<Config, &'static str> {
        args.next();

        let needle = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a query string"),
        };

        let haystack = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a file name"),
        };

        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();

        Ok(Config {
            needle,
            haystack,
            case_sensitive,
        })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let haystack = fs::read_to_string(config.haystack)?; //.expect("Something went wrong reading the file");
    println!("With text:\n{}", haystack);

    let result = if config.case_sensitive {
        search(&config.needle, &haystack)
    } else {
        search_case_insensitive(&config.needle, &haystack)
    };
    for line in result {
        println!("{}", line);
    }
    Ok(())
}

pub fn search<'a>(needle: &str, haystack: &'a str) -> Vec<&'a str> {
    haystack
        .lines()
        .filter(|line| line.contains(needle))
        .collect()
}

pub fn search_case_insensitive<'a>(needle: &str, haystack: &'a str) -> Vec<&'a str> {
    let needle = needle.to_lowercase();
    let mut result = Vec::new();

    for line in haystack.lines() {
        if line.to_lowercase().contains(&needle) {
            result.push(line);
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive() {
        let needle = "duct";
        let haystack = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

        assert_eq!(vec!["safe, fast, productive."], search(needle, haystack));
    }

    #[test]
    fn case_insensitive() {
        let needle = "rUsT";
        let haystack = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";
        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(needle, haystack)
        );
    }
}
