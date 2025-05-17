use std::collections::HashMap;

use sea_orm::{entity::prelude::*, FromJsonQueryResult};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Deserialize, Serialize)]
#[sea_orm(table_name = "posts")]
pub struct Model {
    #[sea_orm(primary_key)]
    #[serde(skip_deserializing)]
    pub id: i32,
    pub title: String,
    #[sea_orm(column_type = "Text")]
    pub text: String,
    pub status: CheckStatus,
    pub params: Params,
    pub kvs: Option<Kvs>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, EnumIter, DeriveActiveEnum)]
#[serde(rename_all = "snake_case")]
#[sea_orm(
    rs_type = "String",
    db_type = "String(StringLen::N(20))",
    rename_all = "snake_case"
)]
pub enum CheckStatus {
    Pending,
    Running,
    Success,
    Failure,
    Error,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, FromJsonQueryResult)]
pub struct Params {
    pub hello: Option<String>,
    pub foo: Option<String>,
}

// #[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, FromJsonQueryResult)]
// pub type Kvs = Option<HashMap<String, String>>;
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, FromJsonQueryResult)]
pub struct Kvs(pub HashMap<String, String>);

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

#[cfg(test)]
mod tests {
    use super::*;
    use sea_orm::*;

    #[tokio::test]
    async fn test_create_todo() {
        // get env vars
        dotenvy::dotenv().ok();
        let db_url = std::env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");
        let conn = Database::connect(&db_url).await.unwrap();

        let form_data = Model {
            id: 0,
            title: "Test".to_string(),
            text: "Test".to_string(),
            status: CheckStatus::Pending,
            params: Params {
                hello: Some("Hello".to_string()),
                foo: Some("Foo".to_string()),
            },
            kvs: None,
        };
        ActiveModel {
            title: Set(form_data.title.to_owned()),
            text: Set(form_data.text.to_owned()),
            status: Set(form_data.status.to_owned()),
            params: Set(form_data.params.to_owned()),
            ..Default::default()
        }
        .save(&conn)
        .await
        .unwrap();
    }

    #[tokio::test]
    async fn test_list_todos() {
        // get env vars
        dotenvy::dotenv().ok();
        let db_url = std::env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");
        let conn = Database::connect(&db_url).await.unwrap();

        let page = 1;
        let posts_per_page = 10;

        let paginator = Entity::find()
            .order_by_asc(Column::Id)
            .paginate(&conn, posts_per_page);
        let num_pages = paginator.num_pages().await.unwrap();

        // Fetch paginated posts
        let posts = paginator
            .fetch_page(page - 1)
            .await
            .map(|p| (p, num_pages))
            .unwrap();

        println!("=> {:#?}", posts);
        println!("=> {}", serde_json::to_string(&posts.0).unwrap());
    }

    #[tokio::test]
    async fn test_create_todo_with_kvs() {
        // get env vars
        dotenvy::dotenv().ok();
        let db_url = std::env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");
        let conn = Database::connect(&db_url).await.unwrap();

        let form_data = Model {
            id: 0,
            title: "Test".to_string(),
            text: "Test".to_string(),
            status: CheckStatus::Pending,
            params: Params {
                hello: Some("Hello".to_string()),
                foo: Some("Foo".to_string()),
            },
            kvs: {
                let mut map = HashMap::new();
                map.insert("Hello".to_string(), "World".to_string());
                Some(Kvs(map))
            },
        };
        ActiveModel {
            title: Set(form_data.title.to_owned()),
            text: Set(form_data.text.to_owned()),
            status: Set(form_data.status.to_owned()),
            params: Set(form_data.params.to_owned()),
            kvs: Set(form_data.kvs.to_owned()),
            ..Default::default()
        }
        .save(&conn)
        .await
        .unwrap();
    }
}
