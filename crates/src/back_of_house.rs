pub fn fix_incorrect_order() {
    cook_order();
    //invoke parent mod function
    super::serve_order();
}

fn cook_order() {}

pub struct Breakfast {
    pub toast: String,
    pub seasonal_fruit: String,
}

impl Breakfast {
    pub fn summer(toast: &str) -> Breakfast {
        Breakfast {
            toast: String::from(toast),
            seasonal_fruit: String::from("peaches"),
        }
    }
}

pub enum Appetizer {
    Soup,
    Salad,
}
