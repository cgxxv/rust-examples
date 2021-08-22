struct User {
    email: String,
    name: String,
}

fn main() {
    let u = User {
        email: "aaa".to_string(),
        name: "bbb".to_string(),
    };

    for (field_name, field_value) in u {
        println!("{:?} {:?}", field_name, field_value);
    }
}
