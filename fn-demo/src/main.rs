fn call_fn_once<F>(f: F)
where
    F: FnOnce(),
{
    f(); // 只能调用一次
}

fn call_fn_mut<F>(mut f: F)
where
    F: FnMut(),
{
    f();
    f(); // 可以多次调用，但要可变借用
}

fn call_fn<F>(f: F)
where
    F: Fn(),
{
    f();
    f();
    f(); // 可以多次调用，只读环境
}

fn main() {
    let name = String::from("Rust");

    // Fn: 只读借用
    let f1 = || {
        println!("Hello, {}", name);
    };
    call_fn(f1);
    call_fn(f1);

    // FnMut: 可变借用
    let mut counter = 0;
    let f2 = || {
        counter += 1;
        println!("Counter: {}", counter);
    };
    call_fn_mut(f2);

    // FnOnce: 拿走所有权
    let s = String::from("Move me!");
    let f3 = || {
        println!("{}", s);
        drop(s);
        // `s` 被 move 进闭包，不能再调用 f3 第二次
    };
    call_fn_once(f3);
}
