use std::{error::Error, net::Ipv4Addr};

use actix_web::{App, HttpServer, Responder, get, middleware::Logger, web};
use utoipa::OpenApi;
use utoipa_swagger_ui::{SwaggerUi, Url};

#[utoipa::path(
    context_path = "/api",
    responses(
        (status = 200, description = "handle repository with nested groups", body = String)
    )
)]
#[get("/api/{orgs:.+}/{repo}")] // `.+` 表示至少一个 org，后面必须跟一个 repo
async fn handle_repo(path: web::Path<(String, String)>) -> impl Responder {
    let (orgs, repo) = path.into_inner();
    let org_list: Vec<&str> = orgs.split('/').collect();

    format!("Orgs: {:?}, Repo: {}", org_list, repo)
}

#[actix_web::main]
async fn main() -> Result<(), impl Error> {
    env_logger::init();

    #[derive(OpenApi)]
    #[openapi(paths(handle_repo))]
    struct ApiDoc;

    #[derive(OpenApi)]
    #[openapi(paths(api1::hello1))]
    struct ApiDoc1;

    #[derive(OpenApi)]
    #[openapi(paths(api2::hello2))]
    struct ApiDoc2;

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .service(
                web::scope("/api")
                    .service(api1::hello1)
                    .service(api2::hello2)
                    .service(handle_repo),
            )
            .service(SwaggerUi::new("/swagger-ui/{_:.*}").urls(vec![
                (Url::new("api", "/api-docs/openapi.json"), ApiDoc::openapi()),
                (
                    Url::new("api1", "/api-docs/openapi1.json"),
                    ApiDoc1::openapi(),
                ),
                (
                    Url::with_primary("api2", "/api-docs/openapi2.json", true),
                    ApiDoc2::openapi(),
                ),
            ]))
    })
    .bind((Ipv4Addr::UNSPECIFIED, 8090))?
    .run()
    .await
}

mod api1 {
    use actix_web::get;

    #[utoipa::path(
        context_path = "/api",
        responses(
            (status = 200, description = "Hello from api 1", body = String)
        )
    )]
    #[get("/api1/hello")]
    pub(super) async fn hello1() -> String {
        "hello from api 1".to_string()
    }
}

mod api2 {
    use actix_web::get;

    #[utoipa::path(
        context_path = "/api",
        responses(
            (status = 200, description = "Hello from api 2", body = String)
        )
    )]
    #[get("/api2/hello")]
    pub(super) async fn hello2() -> String {
        "hello from api 2".to_string()
    }
}
