macro_rules! zoom_and_enhance {
    (struct $name:ident { $($fname:ident : $ftype:ty),* }) => {
        struct $name {
            $($fname : $ftype),*
        }

        impl $name {
            fn field_names() -> &'static [&'static str] {
                static NAMES: &'static [&'static str] = &[$(stringify!($fname)),*];
                NAMES
            }
        }
    }
}

zoom_and_enhance! {
struct Export {
    first_name: String,
    last_name: String,
    gender: String,
    date_of_birth: String,
    address: String
}
}

fn main() {
    println!("{:?}", Export::field_names());
}
