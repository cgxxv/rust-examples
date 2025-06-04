use std::time::Duration;

use futures::FutureExt;
use tokio::time::sleep;
use tokio_util::sync::CancellationToken;

#[tokio::main]
async fn main() {
    let delays = [
        Duration::from_millis(1),
        Duration::from_millis(10),
        Duration::from_millis(20),
        Duration::from_millis(30),
        Duration::from_millis(40),
        Duration::from_millis(50),
    ];

    let mut consecutive_failures = 0;
    let cancel_token = CancellationToken::new();

    loop {
        let delay = if consecutive_failures < delays.len() {
            delays[consecutive_failures]
        } else {
            consecutive_failures = 0;
            delays[delays.len() - 1]
        };
        dbg!(delay);

        let delay_fut = sleep(delay).fuse();
        tokio::pin!(delay_fut);

        tokio::select! {
            _ = cancel_token.cancelled() => {
                dbg!("cancelled");
                break;
            }
            _ = &mut delay_fut => {
                dbg!("delay expired");
                // your retry logic here
            }
        }

        consecutive_failures += 1;
    }
}
