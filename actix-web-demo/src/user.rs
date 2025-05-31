use actix_web::{Responder, get, post, web};

use crate::model::{group, user};

const USERS: &str = "users";

// #[allow(unused)]
// pub(super) fn configure() -> impl FnOnce(&mut ServiceConfig) {
//     |config: &mut ServiceConfig| {
//         config.service(get_user).service(create_user);
//     }
// }

#[allow(unused)]
pub(super) fn configure_utoipa() -> impl FnOnce(&mut utoipa_actix_web::service_config::ServiceConfig)
{
    |config: &mut utoipa_actix_web::service_config::ServiceConfig| {
        config
            .service(get_user)
            .service(get_user_with_group)
            .service(get_user_with_groups)
            .service(create_user)
            .service(get_group)
            .service(get_group_with_user)
            .service(get_group_with_users)
            .service(list_group_with_user)
            .service(list_group_with_users);
    }
}

#[utoipa::path(
    // get,
    // context_path = "/api/users",
    responses(
        (status = 200, description = "User found", body = user::Model),
        (status = 404, description = "User not found")
    ),
    params(("id" = u64, Path, description = "User ID")),
    security(
        ("bearer_token" = [])  // 标记需要认证
    ),
    tag = USERS
)]
#[get("/users/{id}")]
async fn get_user(id: web::Path<u64>) -> impl Responder {
    format!("User ID: {}", id)
}

#[utoipa::path(
    // get,
    // context_path = "/api/users",
    responses(
        (status = 200, description = "User found", body = user::WithGroup),
        (status = 404, description = "User not found")
    ),
    params(("id" = u64, Path, description = "User ID")),
    security(
        ("bearer_token" = [])  // 标记需要认证
    ),
    tag = USERS
)]
#[get("/users/group/{id}")]
async fn get_user_with_group(id: web::Path<u64>) -> impl Responder {
    format!("User ID: {}", id)
}

#[utoipa::path(
    // get,
    // context_path = "/api/users",
    responses(
        (status = 200, description = "User found", body = user::WithGroups),
        (status = 404, description = "User not found")
    ),
    params(("id" = u64, Path, description = "User ID")),
    security(
        ("bearer_token" = [])  // 标记需要认证
    ),
    tag = USERS
)]
#[get("/users/groups/{id}")]
async fn get_user_with_groups(id: web::Path<u64>) -> impl Responder {
    format!("User ID: {}", id)
}

#[utoipa::path(
    // get,
    // context_path = "/api/users",
    responses(
        (status = 200, description = "group found", body = group::Model),
        (status = 404, description = "group not found")
    ),
    params(("id" = u64, Path, description = "User ID")),
    security(
        ("bearer_token" = [])  // 标记需要认证
    ),
    tag = USERS
)]
#[get("/groups/user/{id}")]
async fn get_group(id: web::Path<u64>) -> impl Responder {
    format!("Group ID: {}", id)
}

#[utoipa::path(
    // get,
    // context_path = "/api/users",
    responses(
        (status = 200, description = "group found", body = group::WithUser),
        (status = 404, description = "group not found")
    ),
    params(("id" = u64, Path, description = "User ID")),
    security(
        ("bearer_token" = [])  // 标记需要认证
    ),
    tag = USERS
)]
#[get("/groups/users/{id}")]
async fn get_group_with_user(id: web::Path<u64>) -> impl Responder {
    format!("Group ID: {}", id)
}

#[utoipa::path(
    // get,
    // context_path = "/api/users",
    responses(
        (status = 200, description = "group found", body = (Vec<group::WithUser>, u64)),
        (status = 404, description = "group not found")
    ),
    params(("id" = u64, Path, description = "User ID")),
    security(
        ("bearer_token" = [])  // 标记需要认证
    ),
    tag = USERS
)]
#[get("/groups/user")]
async fn list_group_with_user(id: web::Path<u64>) -> impl Responder {
    format!("Group ID: {}", id)
}

#[utoipa::path(
    // get,
    // context_path = "/api/users",
    responses(
        (status = 200, description = "group found", body = (Vec<group::WithUsers>, u64)),
        (status = 404, description = "group not found")
    ),
    params(("id" = u64, Path, description = "User ID")),
    security(
        ("bearer_token" = [])  // 标记需要认证
    ),
    tag = USERS
)]
#[get("/groups/users")]
async fn list_group_with_users(id: web::Path<u64>) -> impl Responder {
    format!("Group ID: {}", id)
}

#[utoipa::path(
    // get,
    // context_path = "/api/users",
    responses(
        (status = 200, description = "group found", body = group::WithUsers),
        (status = 404, description = "group not found")
    ),
    params(("id" = u64, Path, description = "User ID")),
    security(
        ("bearer_token" = [])  // 标记需要认证
    ),
    tag = USERS
)]
#[get("/groups/{id}")]
async fn get_group_with_users(id: web::Path<u64>) -> impl Responder {
    format!("Group ID: {}", id)
}

#[utoipa::path(
    // post,
    // context_path = "/api/users",
    request_body = user::Model,
    responses(
        (status = 201, description = "User created")
    ),
    security(
        ("bearer_token" = [])  // 标记需要认证
    ),
    tag = USERS
)]
#[post("")]
async fn create_user(user: web::Json<user::Model>) -> impl Responder {
    format!("Created user: {}", user.name)
}
