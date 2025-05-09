use tokio::sync::OnceCell;

#[derive(Default)]
struct MyStruct {
    once: OnceCell<bool>,
}

impl MyStruct {
    fn new() -> Self {
        MyStruct {
            once: OnceCell::new(),
        }
    }

    async fn run(&self) -> bool {
        let res = self
            .once
            .get_or_init(|| async {
                println!("Running...");
                true
            })
            .await;

        *res
    }
}

#[tokio::main]
async fn main() {
    let s = MyStruct::default();

    assert!(!s.once.initialized());

    s.run().await;
    assert!(s.once.initialized());
    s.run().await;
    s.run().await;
    s.run().await;

    println!("Hello, world!");

    assert!(!s.once.initialized());
}
