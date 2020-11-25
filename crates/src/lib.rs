mod front_of_house;

pub fn eat_at_restaurant() {
    //Absolute path
    crate::front_of_house::hosting::add_to_waitlist();

    //Relative path
    front_of_house::hosting::add_to_waitlist();

    //Order a breakfast in the summer with Rye toast
    let mut meal = back_of_house::Breakfast::summer("Rye");
    //Change our mind about what bread we'd like
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);

    //The next line won't compile if we uncomment it; we're not allowed
    //to see or modify the seasonal fruit that comes with the meal
    meal.seasonal_fruit = String::from("blueberries");

    let _order1 = back_of_house::Appetizer::Soup;
    let _order2 = back_of_house::Appetizer::Salad;

    demo::eat_at_restaurant();
    demo_v2::eat_at_restaurant();
    demo_v3::eat_at_restaurant_v2();

    eat_at_restaurant_v3();
    eat_at_restaurant_v4();
    back_of_house::fix_incorrect_order();
}

mod demo;

mod demo_v2;

mod demo_v3;

//re-exporting
use crate::front_of_house::hosting::add_to_waitlist;

pub fn eat_at_restaurant_v3() {
    add_to_waitlist();
    add_to_waitlist();
    add_to_waitlist();
}

pub use crate::front_of_house::hosting;

pub fn eat_at_restaurant_v4() {
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
}

fn serve_order() {}

mod back_of_house;
