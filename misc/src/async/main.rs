use futures::executor::block_on;

async fn hello_world() {
    println!("Hello, World!");
}

fn main() {
    let future = hello_world();
    block_on(future);
}
