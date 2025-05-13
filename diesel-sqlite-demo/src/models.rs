// models.rs
use diesel::backend::Backend;
use diesel::deserialize::{self, FromSql, FromSqlRow};
use diesel::expression::AsExpression;
use diesel::prelude::*;
use diesel::serialize::{self, IsNull, Output, ToSql};
use diesel::sql_types::{Jsonb, Text};
use diesel::{mysql::Mysql, pg::Pg, sqlite::Sqlite};
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

// SQLite (TEXT)
impl ToSql<Text, Sqlite> for ProductDetails {
    fn to_sql(&self, out: &mut Output<'_, '_, Sqlite>) -> serialize::Result {
        let json = serde_json::to_string(self)?;
        out.write_all(json.as_bytes())?;
        Ok(IsNull::No)
    }
}

impl FromSql<Text, Sqlite> for ProductDetails {
    fn from_sql(bytes: diesel::backend::RawValue<'_, Sqlite>) -> deserialize::Result<Self> {
        let s = std::str::from_utf8(bytes.as_bytes())?;
        Ok(serde_json::from_str(s)?)
    }
}

// PostgreSQL (JSONB)
impl ToSql<Jsonb, Pg> for ProductDetails {
    fn to_sql(&self, out: &mut Output<'_, '_, Pg>) -> serialize::Result {
        let json = serde_json::to_vec(self)?;
        out.write_all(&json)?;
        Ok(IsNull::No)
    }
}

impl FromSql<Jsonb, Pg> for ProductDetails {
    fn from_sql(bytes: diesel::backend::RawValue<'_, Pg>) -> deserialize::Result<Self> {
        Ok(serde_json::from_slice(bytes.as_bytes())?)
    }
}

// MySQL (TEXT)
impl ToSql<Text, Mysql> for ProductDetails {
    fn to_sql(&self, out: &mut Output<'_, '_, Mysql>) -> serialize::Result {
        let json = serde_json::to_string(self)?;
        out.write_all(json.as_bytes())?;
        Ok(IsNull::No)
    }
}

impl FromSql<Text, Mysql> for ProductDetails {
    fn from_sql(bytes: diesel::backend::RawValue<'_, Mysql>) -> deserialize::Result<Self> {
        let s = std::str::from_utf8(bytes.as_bytes())?;
        Ok(serde_json::from_str(s)?)
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
pub struct NewProduct {
    pub name: String,
    pub details: ProductDetails,
}
