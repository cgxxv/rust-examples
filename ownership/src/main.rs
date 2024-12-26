use std::str;
use std::string::String;

fn main() {
  let mut s = String::from("Hello, Rustacean!");
  // let mut s = "Hello, Rustacean!";
  let word = first_word(&s);
  println!("word: {}", word);
  s.clear();
}

fn first_word(s: &str) -> &str{
  let bytes = s.as_bytes();

  for (i, &item) in bytes.iter().enumerate() {
    if item == b' '{
      return &s[0..i];
    }
  }

  &s[..]
}
