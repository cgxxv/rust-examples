use actix_web::{
    Error, HttpMessage,
    dev::{Service, ServiceRequest, ServiceResponse, Transform},
    error::ErrorUnauthorized,
    http,
};
use futures::future::{LocalBoxFuture, Ready, ready};
use std::rc::Rc;

use super::jwt::{JwtConfig, validate_token};

pub struct JwtAuth {
    config: Rc<JwtConfig>,
}

impl JwtAuth {
    pub fn new(config: JwtConfig) -> Self {
        Self {
            config: Rc::new(config),
        }
    }
}

impl<S> Transform<S, ServiceRequest> for JwtAuth
where
    S: Service<ServiceRequest, Response = ServiceResponse, Error = Error> + 'static,
    S::Future: 'static,
{
    type Response = ServiceResponse;
    type Error = Error;
    type Transform = JwtAuthMiddleware<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(JwtAuthMiddleware {
            service,
            config: Rc::clone(&self.config),
        }))
    }
}

pub struct JwtAuthMiddleware<S> {
    service: S,
    config: Rc<JwtConfig>,
}

impl<S> Service<ServiceRequest> for JwtAuthMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse, Error = Error> + 'static,
    S::Future: 'static,
{
    type Response = ServiceResponse;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    fn poll_ready(
        &self,
        ctx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Result<(), Self::Error>> {
        self.service.poll_ready(ctx)
    }

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let config = Rc::clone(&self.config);

        // 在创建future前提取必要信息
        let method = req.method().clone();
        let headers = req.headers().clone();

        // 提前处理认证逻辑
        let auth_result = (|| {
            if method == http::Method::OPTIONS {
                return Ok(None);
            }

            let token = headers
                .get(http::header::AUTHORIZATION)
                .and_then(|h| h.to_str().ok())
                .and_then(|s| s.strip_prefix("Bearer "));

            match token {
                Some(token) => validate_token(token, &config.secret)
                    .map(Some)
                    .map_err(|e| ErrorUnauthorized(format!("Invalid token: {}", e))),
                None => Err(ErrorUnauthorized("Missing authorization token")),
            }
        })();

        match auth_result {
            Ok(claims) => {
                if let Some(claims) = claims {
                    req.extensions_mut().insert(claims);
                }
                Box::pin(self.service.call(req))
            }
            Err(e) => Box::pin(async { Err(e) }),
        }
    }
}
