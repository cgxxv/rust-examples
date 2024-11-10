#[derive(Debug)]
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

// struct User {
//     active: bool,
//     username: &str,
//     email: &str,
//     sign_in_count: u64,
// }

fn main() {
    // let user1 = User {
    //     active: true,
    //     username: "someusername123",
    //     email: "someone@example.com",
    //     sign_in_count: 1,
    // };
    tuple_struct();
}

fn tuple_struct() {
    let black = Color(0, 0, 0);
    println!("{:#?}", black);
    println!("{black:#?}");
    println!("{:?}", black);
    println!("{black:?}");
    println!("{}, {}, {}", black.0, black.1, black.2);
    #[allow(unused_variables)]
    let origin = Point(0, 0, 0);
    // println!("{} {}", origin[0], origin[1]);
}
