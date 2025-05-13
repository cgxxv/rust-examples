// models.rs
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use serde_json::{Value, from_str, to_string};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct ProductDetails {
    pub brand: String,
    pub model: String,
    pub specs: HashMap<String, String>,
    pub price: f64,
}

impl From<String> for ProductDetails {
    fn from(s: String) -> Self {
        from_str(&s).expect("Failed to deserialize ProductDetails")
    }
}

impl From<ProductDetails> for String {
    fn from(pd: ProductDetails) -> Self {
        to_string(&pd).expect("Failed to serialize ProductDetails")
    }
}

impl<DB> diesel::serialize::ToSql<diesel::sql_types::Text, DB> for ProductDetails
where
    DB: diesel::backend::Backend,
    String: diesel::serialize::ToSql<diesel::sql_types::Text, DB>,
{
    fn to_sql<'b>(
        &'b self,
        out: &mut diesel::serialize::Output<'b, '_, DB>,
    ) -> diesel::serialize::Result {
        let s = to_string(self)?;
        out.set_value(s);
        Ok(diesel::serialize::IsNull::No)
    }
}

impl<DB> diesel::deserialize::FromSql<diesel::sql_types::Text, DB> for ProductDetails
where
    DB: diesel::backend::Backend,
    String: diesel::deserialize::FromSql<diesel::sql_types::Text, DB>,
{
    fn from_sql(value: DB::RawValue<'_>) -> diesel::deserialize::Result<Self> {
        let s = String::from_sql(value)?;
        Ok(from_str(&s)?)
    }
}

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::products)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Product {
    pub id: i32,
    pub name: String,
    pub details: ProductDetails,
}

#[derive(Insertable)]
#[diesel(table_name = crate::schema::products)]
pub struct NewProduct<'a> {
    pub name: &'a str,
    pub details: &'a str,
}
