fn add_one(x: i32) -> i32 {
    x + 1
}

fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 {
    f(arg) + f(arg)
}

fn main() {
    let answer = do_twice(add_one, 5);
    println!("The answer is: {}", answer);

    demo();
}

fn demo() {
    let list_of_numbers = vec![1, 2, 3];
    let list_of_strings: Vec<String> = list_of_numbers
        .iter()
        .map(|i| i.to_string())
        .collect();
    println!("{:?}", list_of_strings);
    let list_of_strings2: Vec<String> = list_of_numbers
        .iter()
        .map(ToString::to_string)
        .collect();
    println!("{:?}", list_of_strings2);
}

fn demo2() {
    enum Status {
        Value(u32),
        Stop,
    }

    let _list_of_statuses: Vec<Status> = (0u32..20)
        .map(Status::Value)
        .collect();

    //println!("{:?}", list_of_statuses);
}
