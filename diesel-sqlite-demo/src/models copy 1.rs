use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct ProductDetails {
    pub brand: String,
    pub model: String,
    pub specs: HashMap<String, String>,
    pub price: f64,
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
    fn from_sql(bytes: diesel::backend::RawValue<DB>) -> diesel::deserialize::Result<Self> {
        let s = String::from_sql(bytes)?;
        Ok(serde_json::from_str(&s)?)
    }
}

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::products)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Product {
    pub id: i32,
    pub name: String,
    #[diesel(deserialize_as = "ProductDetails")]
    pub details: ProductDetails,
}
