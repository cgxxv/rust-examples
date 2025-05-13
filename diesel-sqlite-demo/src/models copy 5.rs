// models.rs
use diesel::backend::Backend;
use diesel::deserialize::{self, FromSql, FromSqlRow};
use diesel::expression::AsExpression;
use diesel::prelude::*;
use diesel::serialize::{self, Output, ToSql};
use diesel::sql_types::Text;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::io::Write;

#[derive(
    Serialize,
    Deserialize,
    FromSqlRow,
    AsExpression,
    Debug,
    Clone,
    // PartialEq,
    // Eq,
    // Default,
)]
#[diesel(sql_type = Text)]
pub struct ProductDetails {
    pub brand: String,
    pub model: String,
    pub specs: HashMap<String, String>,
    pub price: f64,
}

// impl<DB> diesel::serialize::ToSql<diesel::sql_types::Text, DB> for ProductDetails
// where
//     DB: diesel::backend::Backend,
//     String: diesel::serialize::ToSql<diesel::sql_types::Text, DB>,
// {
//     fn to_sql<'b>(
//         &'b self,
//         out: &mut diesel::serialize::Output<'b, '_, DB>,
//     ) -> diesel::serialize::Result {
//         // serde_json::to_string(self)?.to_sql(out)
//         let s = serde_json::to_string(self)?;
//         out.set_value(&s);
//         Ok(diesel::serialize::IsNull::No)
//     }
// }

impl<DB> diesel::serialize::ToSql<diesel::sql_types::Text, DB> for ProductDetails
where
    DB: diesel::backend::Backend,
{
    fn to_sql(&self, out: &mut diesel::serialize::Output<'_, '_, DB>) -> diesel::serialize::Result {
        let json = serde_json::to_string(self)?;
        <String as ToSql<Text, DB>>::to_sql(&json, out)
    }
}

impl<DB> diesel::deserialize::FromSql<diesel::sql_types::Text, DB> for ProductDetails
where
    DB: diesel::backend::Backend,
    String: diesel::deserialize::FromSql<diesel::sql_types::Text, DB>,
{
    fn from_sql(value: DB::RawValue<'_>) -> diesel::deserialize::Result<Self> {
        let s = String::from_sql(value)?;
        Ok(serde_json::from_str(&s)?)
    }
}

// impl<DB> ToSql<Text, DB> for ProductDetails
// where
//     DB: Backend,
//     String: ToSql<Text, DB>,
// {
//     fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, DB>) -> serialize::Result {
//         let json_str = serde_json::to_string(self).unwrap();
//         json_str.to_sql(out)
//     }
// }

// impl<DB> FromSql<Text, DB> for ProductDetails
// where
//     DB: Backend,
//     String: FromSql<Text, DB>,
// {
//     fn from_sql(bytes: diesel::backend::RawValue<DB>) -> deserialize::Result<Self> {
//         let json_str = String::from_sql(bytes)?;
//         serde_json::from_str(&json_str).map_err(|e| Box::new(e) as _)
//     }
// }

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
pub struct NewProduct {
    pub name: String,
    pub details: ProductDetails,
}
