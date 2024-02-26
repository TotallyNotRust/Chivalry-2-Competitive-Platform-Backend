use std::future::{ready, Ready};

use actix_web::{
    dev::{forward_ready, Extensions, Service, ServiceRequest, ServiceResponse, Transform},
    error::PayloadError,
    http::header::HeaderName,
    Error, HttpMessage,
};
use futures_util::future::LocalBoxFuture;

use crate::{
    lib::database::model::Token,
    utils::tokens::{generate_token, token_to_account},
};
use actix_web::web::Bytes;
use async_std::task;
use futures_util::stream::StreamExt;

// There are two steps in middleware processing.
// 1. Middleware initialization, middleware factory gets called with
//    next service in chain as parameter.
// 2. Middleware's call method gets called with normal request.
pub struct Auth;

// Middleware factory is `Transform` trait
// `S` - type of the next service
// `B` - type of response's body
impl<S, B> Transform<S, ServiceRequest> for Auth
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = AuthMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(AuthMiddleware { service }))
    }
}

pub struct AuthMiddleware<S> {
    service: S,
}

impl<S, B> Service<ServiceRequest> for AuthMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    forward_ready!(service);

    fn call(&self, mut req: ServiceRequest) -> Self::Future {
        println!("{} accessed {}", req.peer_addr().unwrap().ip(), req.path());
        let header_token = req
            .headers()
            .get("Authorization")
            .unwrap()
            .to_str()
            .unwrap()
            .split(" ")
            .collect::<Vec<&str>>()[1];

        let mut token_obj: Option<Token> = None;
        if let Some((acc, mut _token_obj)) = token_to_account(header_token) {
            if _token_obj.is_expired() {
                // Generate token automatically saves the new token, so we just need to check if an error occured (None is returned)
                _token_obj.token = generate_token(&acc).expect("Could not generate new token");

                token_obj = Some(_token_obj.to_owned());
            }
            req.extensions_mut().insert(acc);
        }

        let fut = self.service.call(req);

        Box::pin(async move {
            let mut res = fut.await?;

            if let Some(token_obj) = token_obj {
                res.headers_mut().append(
                    HeaderName::from_bytes("X-New-Token".as_bytes()).unwrap(),
                    token_obj.token.parse().unwrap(),
                );
            }

            Ok(res)
        })
    }
}
