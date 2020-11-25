use std::error::Error;
use std::fs;

pub fn run(config: config::Config) -> Result<(), Box<dyn Error>> {
    let haystack = fs::read_to_string(config.haystack)?; //.expect("Something went wrong reading the file");
    println!("With text:\n{}", haystack);

    let result = if config.case_sensitive {
        search::search(&config.needle, &haystack)
    } else {
        search::search_case_insensitive(&config.needle, &haystack)
    };
    for line in result {
        println!("{}", line);
    }
    Ok(())
}

pub mod config;
mod search;
