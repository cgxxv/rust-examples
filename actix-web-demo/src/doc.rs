use actix_web::{HttpResponse, Responder, get};
use utoipa::{
    Modify, OpenApi,
    openapi::security::{ApiKey, ApiKeyValue, HttpAuthScheme, HttpBuilder, SecurityScheme},
};

use crate::user;

#[get("/api-docs/openapi.json")]
async fn openapi_json() -> HttpResponse {
    HttpResponse::Ok()
        .content_type("application/json")
        .body(ApiDoc::openapi().to_json().unwrap())
}

#[get("/api-docs/openapi.yaml")]
pub async fn openapi_yaml() -> impl Responder {
    let yaml = serde_yaml::to_string(&ApiDoc::openapi()).unwrap();
    HttpResponse::Ok().content_type("text/yaml").body(yaml)
}

#[derive(OpenApi)]
#[openapi(
    // 统一前缀的方式
    // servers(
    //     (url = "/api/v1", description = "API 主服务器"),
    //     (url = "/api/v1/staging", description = "测试服务器"),
    // ),
    components(
        schemas(user::User),
    ),
    tags(
        (name = "orgs", description = "Org management"),
        (name = "users", description = "User management")
    ),
    modifiers(&SecurityAddon)
)]
pub struct ApiDoc;

struct SecurityAddon;

impl Modify for SecurityAddon {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        let components = openapi.components.as_mut().unwrap(); // we can unwrap safely since there already is components registered.
        components.add_security_scheme(
            "api_key",
            SecurityScheme::ApiKey(ApiKey::Header(ApiKeyValue::new("todo_apikey"))),
        );
        components.add_security_scheme(
            "bearer_token",
            SecurityScheme::Http(
                HttpBuilder::new()
                    .scheme(HttpAuthScheme::Bearer)
                    .bearer_format("JWT")
                    .build(),
            ),
        );
    }
}
