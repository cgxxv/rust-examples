fn main() {
    default_sample();
    custom_sample();
    specific_syntax();
    fully_qualified_syntax();
    print_sample();
    newtype_sample();
}

struct Wrapper(Vec<String>);

impl fmt::Display for Wrapper{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result{
        write!(f, "[{}]", self.0.join(", "))
    }
}

fn newtype_sample() {
    let w = Wrapper(vec![String::from("Hello"), String::from("World")]);
    println!("w = {}", w);
}

use std::fmt;

trait OutlinePrint: fmt::Display{
    fn outline_print(&self) {
        let output = self.to_string();
        let len = output.len();
        println!("{}", "*".repeat(len+4));
        println!("*{}*", " ".repeat(len+2));
        println!("* {} *", output);
        println!("*{}*", " ".repeat(len+2));
        println!("{}", "*".repeat(len+4));
    }
}

impl OutlinePrint for Point {}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

fn print_sample() {
    let p = Point{x:1,y:3};
    p.outline_print();
}

trait Animal{
    fn baby_name() -> String;
}

struct Dog;

impl Dog{
    fn baby_name() -> String{
        String::from("Spot")
    }
}

impl Animal for Dog{
    fn baby_name() -> String{
        String::from("puppy")
    }
}

fn fully_qualified_syntax() {
    println!("A baby dog is called a {}", Dog::baby_name());
    println!("A baby dog is called a {}", <Dog as Animal>::baby_name());
}

trait Pilot{
    fn fly(&self);
}

trait Wizard{
    fn fly(&self);
}

struct Human;

impl Pilot for Human{
    fn fly(&self) {
        println!("This is your captain speaking.");
    }
}

impl Wizard for Human{
    fn fly(&self) {
        println!("Up!");
    }
}

impl Human{
    fn fly(&self) {
        println!("*waving arms furiously*");
    }
}

fn specific_syntax() {
    let person = Human;
    Pilot::fly(&person);
    Wizard::fly(&person);
    person.fly();
}

use std::ops::Add;

#[derive(Debug, PartialEq)]
struct Millimeters(u32);
#[derive(Debug, PartialEq)]
struct Meters(u32);

impl Add<Meters> for Millimeters{
    type Output = Millimeters;

    fn add(self, other: Meters) -> Millimeters{
        Millimeters(self.0 + other.0 * 1000)
    }
}

fn custom_sample() {
    let m = Millimeters(2000) + Meters(1);
    assert_eq!(m, Millimeters(3000));
}

#[derive(Debug, PartialEq)]
struct Point{
    x: i32,
    y: i32,
}

impl Add for Point{
    type Output = Point;

    fn add(self, other: Point) -> Point{
        Point{
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

fn default_sample() {
    let p = Point{x:1,y:0} + Point{x:2,y:3};
    assert_eq!(p, Point{x:3,y:3});
}


