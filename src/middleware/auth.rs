use actix_web::{
    dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
    error::ErrorUnauthorized, Error, HttpMessage,
};
use futures::future::{err, ready, LocalBoxFuture, Ready};
use jsonwebtoken::{decode, DecodingKey, Validation};
use crate::core::entities::auth::ClaimsToUserToken;
use crate::config::config_env::Config;
use crate::utils::validations::is_public_route;

pub struct AuthMiddleware;

// Implementação do Transform trait para AuthMiddleware
impl<S, B> Transform<S, ServiceRequest> for AuthMiddleware
where
// Restrições de tipo necessárias para o Service
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Transform = AuthMiddlewareService<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    // Cria uma nova instância do middleware
    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(AuthMiddlewareService { service }))
    }
}

// Serviço do Middleware que faz o trabalho real
pub struct AuthMiddlewareService<S> {
    service: S,
}

// Implementação do Service trait - Onde a lógica principal acontece
impl<S, B> Service<ServiceRequest> for AuthMiddlewareService<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    // Helper fornecido pelo actix para lidar com readiness do serviço
    forward_ready!(service);

    // Método principal que processa cada requisição
    fn call(&self, req: ServiceRequest) -> Self::Future {

        // Carrega as variaveis de ambiente
        let config = req.app_data::<actix_web::web::Data<Config>>().unwrap();

        // Verifica se é a rota do Swagger que não precisa de api_key
        if req.path().starts_with("/api/swagger") {
            // Se for a rota do Swagger, permite acesso sem api_key
        } else {
            // Verifica a api key para todas as outras rotas
            let api_key_header = req.headers().get("api_key");

            match api_key_header {
                Some(api_key_header) => {
                    if api_key_header.to_str().unwrap() != config.api_key {
                        return Box::pin(err(ErrorUnauthorized("wrong api_key")));
                    }
                },
                None => {
                    return Box::pin(async move { Err(ErrorUnauthorized("empty api_key")) });
                }
            };
        }

        // Primeiro verifica se é rota pública
        if is_public_route(req.path()) {
            let fut = self.service.call(req);
            return Box::pin(async move {
                let res = fut.await?;
                Ok(res)
            });
        }

        // Verifica existência do header de autorização
        let auth_header = req.headers().get("Authorization");

        let auth_header = match auth_header {
            Some(header) => header.to_str().unwrap_or_default(),
            None => {
                return Box::pin(async move {
                    Err(ErrorUnauthorized("No authorization header"))
                })
            }
        };

        // Verifica formato do token
        if !auth_header.starts_with("Bearer ") {
            return Box::pin(async move {
                Err(ErrorUnauthorized("Invalid authorization header"))
            });
        }

        // Decodifica e valida o token
        let token = &auth_header[7..];
        let token_data = match decode::<ClaimsToUserToken>(
            token,
            &DecodingKey::from_secret(config.jwt_secret.as_bytes()),
            &Validation::default(),
        ) {
            Ok(data) => data,
            Err(_) => {
                return Box::pin(async move {
                    Err(ErrorUnauthorized("Invalid token"))
                })
            }
        };

        // Se chegou aqui, token é válido e permissões estão ok
        req.extensions_mut().insert(token_data.claims);

        let fut = self.service.call(req);
        Box::pin(async move {
            let res = fut.await?;
            Ok(res)
        })
    }
}
