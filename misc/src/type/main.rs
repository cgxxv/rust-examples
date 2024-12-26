//高级类型自定义
type Thunk = Box<dyn Fn() + Send + 'static>;

let f: Thunk = Box::new(|| println!("hi"));

fn takes_long_type(f: Thunk) {}

fn returns_long_type() -> Thunk{}

//从不返回never type
fn bar() -> !{}

let guess: u32 = match guess.trim().parse() {
    Ok(num) => num,
    Err(_) => continue,//此处返回值是!
}

impl<T> Option<T> {
    pub fn unwrap(self) -> T{
        Some(val) => val,
        None => panic!("called `Option::unwrap()` on a `None` value"),
    }
}

print!("forever");

loop {
    print!("and ever");
}
