use actix_web::dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform};
use actix_web::error::{ErrorBadRequest, ErrorInternalServerError, ErrorNetworkAuthenticationRequired};
use actix_web::Error;
use actix_web_lab::__reexports::futures_util::future::LocalBoxFuture;
use jsonwebtokens_cognito::KeySet;
use log::{error, info};
use std::future::{ready, Ready};

pub struct TokenVerifier;

impl<S, B> Transform<S, ServiceRequest> for TokenVerifier
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Transform = TokenVerifierMiddleware<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(TokenVerifierMiddleware { service }))
    }
}

pub struct TokenVerifierMiddleware<S> {
    service: S,
}

impl<S, B> Service<ServiceRequest> for TokenVerifierMiddleware<S>
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
        let headers = req
            .headers()
            .clone();

        let future = self.service.call(req);

        Box::pin(async move {
            let headers = headers.clone();
            let keyset =
                KeySet::new("us-east-1", "us-east-1_AFyaKUc54")
                    .map_err(|e| ErrorInternalServerError(format!("Failed to create JWT key set.\n{}", e)))?;

            let verifier = keyset
                .new_access_token_verifier(&["2k4mvc8v79o3b1vnvvmtgl36g6"])
                .build()
                .map_err(|e| ErrorInternalServerError(format!("Failed to build access token verifier.\n{}", e)))?;

            let auth_header = headers
                .get("Authorization")
                .ok_or_else(|| ErrorBadRequest("No 'Authorization' header provided."))?;

            let token = auth_header.to_str()
                .map_err(|e| ErrorInternalServerError(e))?
                .strip_prefix("Bearer ")
                .ok_or_else(|| ErrorBadRequest("Token not prefixed with 'Bearer'."))?;

            let verified = keyset.verify(&token, &verifier).await;

            if let Err(e) = verified {
                error!("Token verification failed");
                return Err(ErrorNetworkAuthenticationRequired(e));
            }

            info!("Token verified successfully");

            Ok(future.await?)
        })
    }
}
