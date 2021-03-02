#[macro_export]
macro_rules! vec {
    ($($x:expr),*) => {
        {
            let mut temp_vec = Vec::new();
            $(
                temp_vec.push($x);
            )*
            temp_vec
        }
    };
}

/*
vec![1,2,3];
let mut temp_vec = Vec::new();
temp_vec.push(1);
temp_vec.push(2);
temp_vec.push(3);
temp_vec
*/

/*
//过程宏
use proc_macro;

#[some_attribute]
pub fn some_name(input: TokenStream) -> TokenStream{}
*/

use hello_macro::HelloMacro;

struct Pancakes;

impl HelloMacro for Pancakes{
    fn hello_macro() {
        println!("Hello, Macro! My name is Pancakes!");
    }
}

fn main() {
    Pancakes::hello_macro();
}
