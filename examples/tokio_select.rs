use tokio::time::{self, Duration};
use tokio::signal;

async fn some_async_work() {
    // do work
}

#[tokio::main]
async fn main() {
    let sleep = time::sleep(Duration::from_millis(50));
    tokio::pin!(sleep);

    tokio::select! {
        _ = signal::ctrl_c() => {
            println!("<<<<<=====");
        }
        _ = &mut sleep, if !sleep.is_elapsed() => {
            println!("operation timed out");
        }
        _ = some_async_work() => {
            println!("operation completed");
        }
    }
}
