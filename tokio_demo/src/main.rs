//use std::sync::Mutex;
//use parking_lot::Mutex;
//use async_std::sync::Mutex;
use tokio::sync::Mutex;
use tokio::time::{sleep, Duration};
use tokio_stream::{self as stream, StreamExt};

#[tokio::main]
async fn main() {
    let mtx = Mutex::new(0);

    tokio::join!(work(&mtx), work(&mtx));

    //println!("{}", *mtx.lock().unwrap());
    println!("=================================");

    let mut stream1 = stream::iter(vec![1, 2, 3]);
    let mut stream2 = stream::iter(vec![4, 5, 6]);

    let mut values = vec![];

    loop {
        tokio::select! {
            Some(v) = stream1.next() => values.push(v),
            Some(v) = stream2.next() => values.push(v),
            else => break,
        }
    }

    values.sort();
    assert_eq!(&[1, 2, 3, 4, 5, 6], &values[..]);
}

async fn work(mtx: &Mutex<i32>) {
    println!("lock");
    {
        let mut v = mtx.lock().await;
        println!("locked");
        // slow redis network request
        sleep(Duration::from_millis(100)).await;
        *v += 1;
    }
    println!("unlock")
}
