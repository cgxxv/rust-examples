use std::collections::HashMap;

use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::products)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
struct Product {
    id: i32,
    name: String,
    #[diesel(deserialize_as = "ProductDetails")]
    details: ProductDetails,
}

#[derive(Debug, Serialize, Deserialize)]
struct ProductDetails {
    brand: String,
    model: String,
    specs: HashMap<String, String>,
    price: f64,
}

impl From<Value> for ProductDetails {
    fn from(value: Value) -> Self {
        serde_json::from_value(value).unwrap()
    }
}

impl<DB> diesel::deserialize::FromSql<diesel::sql_types::Text, DB> for ProductDetails
where
    DB: diesel::backend::Backend,
    String: diesel::deserialize::FromSql<diesel::sql_types::Text, DB>,
{
    fn from_sql(
        bytes: diesel::backend::Backend::RawValue<DB>,
    ) -> diesel::deserialize::Result<Self> {
        let s = String::from_sql(bytes)?;
        Ok(serde_json::from_str(&s)?)
    }
}
