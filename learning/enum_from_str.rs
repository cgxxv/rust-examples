use std::str::FromStr;

#[derive(Debug, PartialEq)]
enum Foo {
    Bar,
    Baz,
    Bat,
    Quux,
}

impl FromStr for Foo {
    type Err = ();

    fn from_str(input: &str) -> Result<Foo, Self::Err> {
        match input {
            "Bar" => Ok(Foo::Bar),
            "Baz" => Ok(Foo::Baz),
            "Bat" => Ok(Foo::Bat),
            "Quux" => Ok(Foo::Quux),
            _ => Err(()),
        }
    }
}

fn main() {
    // Use it like this
    let f = Foo::from_str("Baz").unwrap();
    assert_eq!(f, Foo::Baz);
    println!("{:#?}", f)
}
