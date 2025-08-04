use actix_web::{
    Responder, get,
    web::{self, ServiceConfig},
};

const ORGS: &str = "orgs";

#[allow(unused)]
pub(super) fn configure() -> impl FnOnce(&mut ServiceConfig) {
    |config: &mut ServiceConfig| {
        config.service(handle_repo).service(handle_orgs);
    }
}

#[allow(unused)]
pub(super) fn configure_utoipa() -> impl FnOnce(&mut utoipa_actix_web::service_config::ServiceConfig)
{
    |config: &mut utoipa_actix_web::service_config::ServiceConfig| {
        config.service(
            utoipa_actix_web::scope("/{orgs:.+}")
                .service(handle_repo)
                .service(handle_orgs),
        );
    }
}

#[utoipa::path(
    responses(
        (status = 200, description = "handle repository with nested groups", body = String)
    ),
    params(
        ("orgs" = String, Path, description = "组织路径（如 'a/b/c'）"),
        ("repo" = String, Path, description = "仓库名")
    ),
    extensions(
        ("x-openapi-doc" = json!(cfg!(debug_assertions))),
    ),
    tag = ORGS,
)]
#[get("/{repo}")] // `.+` 表示至少一个 org，后面必须跟一个 repo
async fn handle_repo(path: web::Path<(String, String)>) -> impl Responder {
    let (orgs, repo) = path.into_inner();
    let org_list: Vec<&str> = orgs.split('/').collect();

    format!("Orgs: {:?}, Repo: {}", org_list, repo)
}

#[utoipa::path(
    // context_path = "/api/org",
    responses(
        (status = 200, description = "handle nested organizations", body = String)
    ),
    params(
        ("orgs" = String, Path, description = "组织路径（如 'a/b/c'）")
    ),
    tag = ORGS
)]
#[get("")]
async fn handle_orgs(path: web::Path<String>) -> impl Responder {
    let orgs = path.into_inner();
    let org_list: Vec<&str> = orgs.split('/').collect();

    format!("Orgs: {:?}", org_list)
}
