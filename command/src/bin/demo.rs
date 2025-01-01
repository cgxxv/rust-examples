use which::{which, which_global};

fn main() {
    println!("{}", std::env::var("PATH").unwrap());

    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::TRACE)
        .init();

    // tracing::trace!("has a path seperator, so only CWD will be searched.");

    let program = "demo";
    let command = which(program); //.unwrap();
    println!("=> {:#?}", command);
    let command = command.unwrap();
    let command = command.to_str().unwrap();
    println!("=> {command}");

    let command = which_global(program).unwrap();
    let command = command.to_str().unwrap();
    println!("=> {command}");
}
