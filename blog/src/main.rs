use blog::Post;
fn main() {
    println!("Hello, world!");

    let mut post = Post::new();
    post.add_text("I ate a salad for launch today");

    let post = post.request_review();

    let post = post.approve();
    assert_eq!("I ate a salad for launch today", post.content());
}
