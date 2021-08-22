use serde::Serialize;
use std::convert::Infallible;
use std::sync::Arc;
//use std::sync::RwLock;
//use std::sync::Mutex;
//use tokio::sync::Mutex;
use tokio::sync::RwLock;
//use async_std::sync::Mutex;
//use parking_lot::Mutex;
//use parking_lot::RwLock;
use warp::Filter;

type Lock = Arc<RwLock<String>>;

#[derive(Serialize)]
enum T {
    A,
}

//parking_lot::Mutex  20869
//parking_lot::RwLock 21105
//tokio::sync::Mutex  23449
//tokio::sync::RwLock 21093
//std::sync::Mutex    21263
//std::sync::RwLock   21168
//async_std::sync::Mutex 21880

async fn hello(lock: Lock) -> Result<impl warp::Reply, Infallible> {
    lock.read().await;
    Ok(warp::reply::json(&T::A))
}

#[tokio::main]
async fn main() {
    let lock = Arc::new(RwLock::new("mutex".to_owned()));
    let routes = warp::any().map(move || lock.clone()).and_then(hello);
    warp::serve(routes).run(([0, 0, 0, 0], 3030)).await;
}
