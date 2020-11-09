use std::string::String;

fn main() {
    let mut s1 = String::from("hello");
    let len = calculate_length(&s1);
    println!("The length of '{}' is {}.", s1, len);
    change(&mut s1);
    let len2 = calculate_length(&s1);
    println!("The length of '{}' is {}.", s1, len2);
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}
