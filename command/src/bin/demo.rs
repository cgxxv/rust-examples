use which::{which, which_global};

fn main() {
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
