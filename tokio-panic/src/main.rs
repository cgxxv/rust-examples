use std::panic;

#[tokio::main]
async fn main() {
    let err = tokio::spawn(async {
        panic!("boom");
    })
    .await
    .unwrap_err();

    println!("test option message : {}", Some("123"));

    if let Ok(reason) = err.try_into_panic() {
        // Resume the panic on the main task
        panic::resume_unwind(reason);
    }
}
