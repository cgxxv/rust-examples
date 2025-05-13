use std::collections::HashMap;

// main.rs
use crate::models::{NewProduct, Product};
use crate::schema::products;
use diesel::prelude::*;
use models::ProductDetails;
use serde_json::json;

pub mod models;
pub mod schema;

fn main() {
    let conn = &mut establish_connection();

    // let new_product = NewProduct {
    //     name: "Tablet",
    //     details: json!({
    //         "brand": "Samsung",
    //         "model": "Galaxy Tab S8",
    //         "specs": {
    //             "storage": "256GB",
    //             "display": "11-inch"
    //         },
    //         "price": 649.99
    //     })
    //     .as_str()
    //     .unwrap(),
    // };

    let new_product = NewProduct {
        name: "iPhone 15".to_string(),
        details: ProductDetails {
            brand: "Apple".to_string(),
            model: "15 Pro".to_string(),
            specs: {
                let mut map = HashMap::new();
                map.insert("color".into(), "black".into());
                map.insert("storage".into(), "256GB".into());
                map
            },
            price: 999.99,
        },
    };

    diesel::insert_into(products::table)
        .values(&new_product)
        .execute(conn)
        .expect("Error inserting product");

    let products = products::table
        .select(Product::as_select()) // 明确指定选择
        .load(conn)
        .expect("Error loading products");

    for product in products {
        println!("Product: {}", product.name);
        println!("Brand: {}", product.details.brand);
        println!("Price: ${}", product.details.price);
    }
}

pub fn establish_connection() -> SqliteConnection {
    SqliteConnection::establish("data.sqlite3")
        .unwrap_or_else(|e| panic!("Failed to connect: {}", e))
}
