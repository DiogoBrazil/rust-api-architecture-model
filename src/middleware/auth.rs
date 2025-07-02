use actix_web::{
    dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
    error::ErrorUnauthorized, Error, HttpMessage, web,
};
use futures::future::{err, ok, Ready, LocalBoxFuture};
use jsonwebtoken::{decode, DecodingKey, Validation};
use crate::core::entities::auth::ClaimsToUserToken;
use crate::config::config_env::Config;
use crate::utils::validations::is_public_route;

pub struct AuthMiddleware;

impl<S, B> Transform<S, ServiceRequest> for AuthMiddleware
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Transform = AuthMiddlewareService<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(AuthMiddlewareService { service })
    }
}

pub struct AuthMiddlewareService<S> {
    service: S,
}

impl<S, B> Service<ServiceRequest> for AuthMiddlewareService<S>
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
        let config = req.app_data::<web::Data<Config>>().unwrap();

        if let Err(e) = self.verify_api_key(&req, config) {
            return Box::pin(err(e));
        }

        if is_public_route(req.path()) {
            return Box::pin(self.service.call(req));
        }

        match self.verify_jwt_token(&req, config) {
            Ok(claims) => {
                req.extensions_mut().insert(claims);
                Box::pin(self.service.call(req))
            }
            Err(e) => Box::pin(err(e)),
        }
    }
}

impl<S> AuthMiddlewareService<S> {
    fn verify_api_key(&self, req: &ServiceRequest, config: &Config) -> Result<(), Error> {
        if req.path().starts_with("/api/swagger") {
            return Ok(());
        }

        match req.headers().get("api_key") {
            Some(api_key_header) => {
                if api_key_header.to_str().unwrap_or_default() != config.api_key {
                    Err(ErrorUnauthorized("wrong api_key"))
                } else {
                    Ok(())
                }
            }
            None => Err(ErrorUnauthorized("empty api_key")),
        }
    }

    fn verify_jwt_token(&self, req: &ServiceRequest, config: &Config) -> Result<ClaimsToUserToken, Error> {
        let auth_header = req.headers().get("Authorization")
            .and_then(|h| h.to_str().ok())
            .unwrap_or_default();

        if !auth_header.starts_with("Bearer ") {
            return Err(ErrorUnauthorized("Invalid authorization header"));
        }

        let token = &auth_header[7..];
        decode::<ClaimsToUserToken>(
            token,
            &DecodingKey::from_secret(config.jwt_secret.as_bytes()),
            &Validation::default(),
        )
        .map(|data| data.claims)
        .map_err(|_| ErrorUnauthorized("Invalid token"))
    }
}
