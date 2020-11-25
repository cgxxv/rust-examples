use std::env;

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
