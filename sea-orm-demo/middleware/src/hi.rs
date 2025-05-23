use std::future::{Ready, ready};

use actix_web::{
    Error,
    dev::{Service, ServiceRequest, ServiceResponse, Transform, forward_ready},
};
use futures_util::future::LocalBoxFuture;

#[derive(Debug, Clone)]
pub enum Role {
    Saler,
    Manager,
    Owner,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Permission {
    Check,
    Publish,
    All,
}

// There are two steps in middleware processing.
// 1. Middleware initialization, middleware factory gets called with
//    next service in chain as parameter.
// 2. Middleware's call method gets called with normal request.
pub struct SayHi {
    name: String,
    role: Role,
    perms: Vec<Permission>,
}

impl SayHi {
    pub fn new(name: &str, role: Role, perms: Vec<Permission>) -> Self {
        Self {
            name: name.to_string(),
            role,
            perms,
        }
    }
}

// Middleware factory is `Transform` trait
// `S` - type of the next service
// `B` - type of response's body
impl<S, B> Transform<S, ServiceRequest> for SayHi
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = SayHiMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(SayHiMiddleware {
            service,
            name: self.name.clone(),
            role: self.role.clone(),
            perms: self.perms.clone(),
        }))
    }
}

pub struct SayHiMiddleware<S> {
    service: S,
    name: String,
    role: Role,
    perms: Vec<Permission>,
}

impl<S, B> Service<ServiceRequest> for SayHiMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let name = self.name.clone();
        let role = self.role.clone();
        let perms = self.perms.clone();
        println!(
            "Hi from {}. role: {:?}, perms: {:?} You requested: {}",
            name,
            role,
            perms,
            req.path()
        );

        let fut = self.service.call(req);

        Box::pin(async move {
            let res = fut.await?;

            println!(
                "Hi from {} response, role: {:#?}, perms: {:#?}",
                name, role, perms
            );
            Ok(res)
        })
    }
}
