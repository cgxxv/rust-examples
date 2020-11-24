use std::collections::HashMap;

fn main() {
    println!("Hello, world!");

    let mut map = HashMap::new();
    map.insert(1, 2);

    println!("{}", rangle());
}

use rand::Rng;

fn rangle() -> i32 {
    let secret_number = rand::thread_rng().gen_range(1, 101);
    secret_number
}

// use std::fmt;
// use std::io;

// fn function1() -> fmt::Result {}
// fn function2() -> io::Result<()> {}

// use std::fmt::Result;
// use std::io::Result as IoResult;

// fn function3() -> Result {}
// fn function4() -> IoResult<()> {}
