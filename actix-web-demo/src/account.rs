use std::collections::HashMap;

use actix_web::{HttpResponse, Responder, get, web};
use actix_web_httpauth::extractors::bearer::BearerAuth;

use crate::auth::jwt_v2::{generate_jwt, validate_jwt};

const ACCOUNT: &str = "account";

#[allow(unused)]
pub(super) fn configure_utoipa() -> impl FnOnce(&mut utoipa_actix_web::service_config::ServiceConfig)
{
    |config: &mut utoipa_actix_web::service_config::ServiceConfig| {
        config.service(protected).service(login);
    }
}

#[utoipa::path(
    // get,
    // context_path = "/api/users",
    responses(
        (status = 200, description = "User found", body = String),
        (status = 404, description = "User not found")
    ),
    security(
        ("bearer_token" = [])  // 标记需要认证
    ),
    tag = ACCOUNT
)]
#[get("/protected")]
async fn protected(auth: BearerAuth) -> impl Responder {
    match validate_jwt(auth.token()) {
        Ok(claims) => HttpResponse::Ok().body(format!("Authenticated! User ID: {}", claims.sub)),
        Err(_) => HttpResponse::Unauthorized().body("Invalid token"),
    }
}

#[utoipa::path(
    // get,
    // context_path = "/api/users",
    responses(
        (status = 200, description = "User found", body = String),
        (status = 404, description = "User not found")
    ),
    params(("username" = String, Query, description = "Username")),
    tag = ACCOUNT
)]
#[get("/login")]
async fn login(params: web::Query<HashMap<String, String>>) -> impl Responder {
    let username = params.get("username").cloned().unwrap_or_default();
    println!("=> username: {}", username);
    if username.is_empty() {
        return HttpResponse::BadRequest().json("Username cannot be empty");
    }
    let token = generate_jwt(&username);
    HttpResponse::Ok().json(serde_json::json!({ "token": token }))
}
