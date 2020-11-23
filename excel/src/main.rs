// use crate::Config;
use std::env;
use std::process;

fn main() {
    //let args: Vec<String> = env::args().collect();

    let config = Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    //println!("{:?}", args);
    println!("{:?}", config);

    if let Err(e) = Config::run(config) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}