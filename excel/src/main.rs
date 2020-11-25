use excel::{self, config};
use std::env;
use std::process;

fn main() {
    //let args: Vec<String> = env::args().collect();

    let config = config::Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    //println!("{:?}", args);
    println!("{:?}", config);

    if let Err(e) = excel::run(config) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}
