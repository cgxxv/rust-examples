use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use serde_json::from_str;
use std::collections::HashMap;

// 你的自定义结构体
#[derive(Debug, Serialize, Deserialize)]
pub struct ProductDetails {
    pub brand: String,
    pub model: String,
    pub specs: HashMap<String, String>,
    pub price: f64,
}

// 为 ProductDetails 实现 FromSql
impl<DB> diesel::deserialize::FromSql<diesel::sql_types::Text, DB> for ProductDetails
where
    DB: diesel::backend::Backend,
    String: diesel::deserialize::FromSql<diesel::sql_types::Text, DB>,
{
    fn from_sql(value: DB::RawValue<'_>) -> diesel::deserialize::Result<Self> {
        let s = String::from_sql(value)?;
        from_str(&s).map_err(Into::into)
    }
}

// 数据库模型
#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::products)]
// #[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Product {
    pub id: i32,
    pub name: String,
    pub details: ProductDetails,
}
