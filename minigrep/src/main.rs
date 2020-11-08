use minigrep::Config;
use std::env;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    println!("{:?}", args);
    println!("{:?}", config);

    if let Err(e) = minigrep::run(config) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}

/*
fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.haystack)?; //.expect("Something went wrong reading the file");
    println!("With text:\n{}", contents);
    Ok(())
}

struct Config {
    needle: String,
    haystack: String,
}

impl Config {
    fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }
        let needle = args[1].clone();
        let haystack = args[2].clone();
        Ok(Config { needle, haystack })
    }
}
*/
