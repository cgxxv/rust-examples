use std::{
    error::Error,
    future::{self, Ready},
    net::Ipv4Addr,
};

use actix_web::{
    App, HttpResponse, HttpServer,
    dev::{Service, ServiceRequest, ServiceResponse, Transform},
    middleware::Logger,
};
use actix_web_httpauth::middleware::HttpAuthentication;
use futures::future::LocalBoxFuture;
use serde::{Deserialize, Serialize};
use utoipa::{
    Modify, OpenApi, ToSchema,
    openapi::security::{ApiKey, ApiKeyValue, HttpAuthScheme, HttpBuilder, SecurityScheme},
};
use utoipa_actix_web::AppExt;
use utoipa_rapidoc::RapiDoc;
use utoipa_redoc::{Redoc, Servable};
use utoipa_scalar::{Scalar, Servable as ScalarServable};
use utoipa_swagger_ui::SwaggerUi;

mod account;
mod auth;
mod org;
mod user;

const API_KEY_NAME: &str = "todo_apikey";
const API_KEY: &str = "utoipa-rocks";

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
struct ApiDoc;

/// Todo endpoint error responses
#[derive(Serialize, Deserialize, Clone, ToSchema)]
pub enum ErrorResponse {
    /// When Todo is not found by search term.
    NotFound(String),
    /// When there is a conflict storing a new todo.
    Conflict(String),
    /// When todo endpoint was called without correct credentials
    Unauthorized(String),
}

#[actix_web::main]
async fn main() -> Result<(), impl Error> {
    env_logger::init();

    let jwt_config = auth::jwt::JwtConfig::new("your-secret-key".to_string(), 24);

    HttpServer::new(move || {
        // 统一前缀的方式
        // let app = App::new().wrap(Logger::default()).service(
        //     web::scope("/api/v1")
        //         .service(web::scope("/orgs").configure(org::configure()))
        //         .service(web::scope("/users").configure(user::configure())),
        // );
        // app.into_utoipa_app()
        //     .openapi(ApiDoc::openapi())
        //     .openapi_service(|api| Redoc::with_url("/redoc", api))
        //     .openapi_service(|api| {
        //         SwaggerUi::new("/swagger-ui/{_:.*}").url("/api-docs/openapi.json", api)
        //     })
        //     // There is no need to create RapiDoc::with_openapi because the OpenApi is served
        //     // via SwaggerUi. Instead we only make rapidoc to point to the existing doc.
        //     //
        //     // If we wanted to serve the schema, the following would work:
        //     // .openapi_service(|api| RapiDoc::with_openapi("/api-docs/openapi2.json", api).path("/rapidoc"))
        //     .map(|app| app.service(RapiDoc::new("/api-docs/openapi.json").path("/rapidoc")))
        //     .openapi_service(|api| Scalar::with_url("/scalar", api))
        //     .into_app()

        let auth_middleware = auth::middleware::JwtAuth::new(jwt_config.clone());
        App::new()
            .into_utoipa_app()
            .openapi(ApiDoc::openapi())
            .map(|app| app.wrap(Logger::default()))
            .service(
                utoipa_actix_web::scope("/api/v1")
                    .service(
                        utoipa_actix_web::scope("/account").configure(account::configure_utoipa()),
                    )
                    .service(
                        utoipa_actix_web::scope("/orgs")
                            .wrap(HttpAuthentication::bearer(auth::jwt_v2::validator))
                            .configure(org::configure_utoipa()),
                    )
                    .service(
                        utoipa_actix_web::scope("/users")
                            .wrap(auth_middleware)
                            .configure(user::configure_utoipa()),
                    ),
            )
            .openapi_service(|api| Redoc::with_url("/redoc", api))
            .openapi_service(|api| {
                SwaggerUi::new("/swagger-ui/{_:.*}").url("/api-docs/openapi.json", api)
            })
            // There is no need to create RapiDoc::with_openapi because the OpenApi is served
            // via SwaggerUi. Instead we only make rapidoc to point to the existing doc.
            //
            // If we wanted to serve the schema, the following would work:
            // .openapi_service(|api| RapiDoc::with_openapi("/api-docs/openapi2.json", api).path("/rapidoc"))
            .map(|app| app.service(RapiDoc::new("/api-docs/openapi.json").path("/rapidoc")))
            .openapi_service(|api| Scalar::with_url("/scalar", api))
            .into_app()
    })
    .bind((Ipv4Addr::UNSPECIFIED, 8088))?
    .run()
    .await
}

struct SecurityAddon;

impl Modify for SecurityAddon {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        let components = openapi.components.as_mut().unwrap(); // we can unwrap safely since there already is components registered.
        // components.add_security_scheme(
        //     "api_key",
        //     SecurityScheme::ApiKey(ApiKey::Header(ApiKeyValue::new("todo_apikey"))),
        // );
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

/// Require api key middleware will actually require valid api key
struct RequireApiKey;

impl<S> Transform<S, ServiceRequest> for RequireApiKey
where
    S: Service<
            ServiceRequest,
            Response = ServiceResponse<actix_web::body::BoxBody>,
            Error = actix_web::Error,
        >,
    S::Future: 'static,
{
    type Response = ServiceResponse<actix_web::body::BoxBody>;
    type Error = actix_web::Error;
    type Transform = ApiKeyMiddleware<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        future::ready(Ok(ApiKeyMiddleware {
            service,
            log_only: false,
        }))
    }
}

/// Log api key middleware only logs about missing or invalid api keys
struct LogApiKey;

impl<S> Transform<S, ServiceRequest> for LogApiKey
where
    S: Service<
            ServiceRequest,
            Response = ServiceResponse<actix_web::body::BoxBody>,
            Error = actix_web::Error,
        >,
    S::Future: 'static,
{
    type Response = ServiceResponse<actix_web::body::BoxBody>;
    type Error = actix_web::Error;
    type Transform = ApiKeyMiddleware<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        future::ready(Ok(ApiKeyMiddleware {
            service,
            log_only: true,
        }))
    }
}

struct ApiKeyMiddleware<S> {
    service: S,
    log_only: bool,
}

impl<S> Service<ServiceRequest> for ApiKeyMiddleware<S>
where
    S: Service<
            ServiceRequest,
            Response = ServiceResponse<actix_web::body::BoxBody>,
            Error = actix_web::Error,
        >,
    S::Future: 'static,
{
    type Response = ServiceResponse<actix_web::body::BoxBody>;
    type Error = actix_web::Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, actix_web::Error>>;

    fn poll_ready(
        &self,
        ctx: &mut core::task::Context<'_>,
    ) -> std::task::Poll<Result<(), Self::Error>> {
        self.service.poll_ready(ctx)
    }

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let response = |req: ServiceRequest, response: HttpResponse| -> Self::Future {
            Box::pin(async { Ok(req.into_response(response)) })
        };

        match req.headers().get(API_KEY_NAME) {
            Some(key) if key != API_KEY => {
                if self.log_only {
                    log::debug!("Incorrect api api provided!!!")
                } else {
                    return response(
                        req,
                        HttpResponse::Unauthorized().json(ErrorResponse::Unauthorized(
                            String::from("incorrect api key"),
                        )),
                    );
                }
            }
            None => {
                if self.log_only {
                    log::debug!("Missing api key!!!")
                } else {
                    return response(
                        req,
                        HttpResponse::Unauthorized()
                            .json(ErrorResponse::Unauthorized(String::from("missing api key"))),
                    );
                }
            }
            _ => (), // just passthrough
        }

        if self.log_only {
            log::debug!("Performing operation")
        }

        let future = self.service.call(req);

        Box::pin(async move {
            let response = future.await?;

            Ok(response)
        })
    }
}
