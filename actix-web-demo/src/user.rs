use actix_web::{
    Responder, get, post,
    web::{self, ServiceConfig},
};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

const USERS: &str = "users";

#[allow(unused)]
pub(super) fn configure() -> impl FnOnce(&mut ServiceConfig) {
    |config: &mut ServiceConfig| {
        config.service(get_user).service(create_user);
    }
}

#[allow(unused)]
pub(super) fn configure_utoipa() -> impl FnOnce(&mut utoipa_actix_web::service_config::ServiceConfig)
{
    |config: &mut utoipa_actix_web::service_config::ServiceConfig| {
        config.service(get_user).service(create_user);
    }
}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct User {
    id: u64,
    name: String,
}

#[utoipa::path(
    // get,
    // context_path = "/api/users",
    responses(
        (status = 200, description = "User found", body = User),
        (status = 404, description = "User not found")
    ),
    params(("id" = u64, Path, description = "User ID")),
    security(
        ("bearer_token" = [])  // 标记需要认证
    ),
    tag = USERS
)]
#[get("/{id}")]
async fn get_user(id: web::Path<u64>) -> impl Responder {
    format!("User ID: {}", id)
}

#[utoipa::path(
    // post,
    // context_path = "/api/users",
    request_body = User,
    responses(
        (status = 201, description = "User created")
    ),
    security(
        ("bearer_token" = [])  // 标记需要认证
    ),
    tag = USERS
)]
#[post("")]
async fn create_user(user: web::Json<User>) -> impl Responder {
    format!("Created user: {}", user.name)
}
