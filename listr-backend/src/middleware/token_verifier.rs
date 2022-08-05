use actix_web::dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform};
use actix_web::error::ErrorNetworkAuthenticationRequired;
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
        let keyset =
            KeySet::new("us-east-1", "us-east-1_AFyaKUc54").expect("Failed to create JWT keyset");

        let verifier = keyset
            .new_access_token_verifier(&["2k4mvc8v79o3b1vnvvmtgl36g6"])
            .build()
            .expect("Failed to build token verifier");

        let auth_header = req
            .headers()
            .get("Authorization")
            .expect("No auth header.")
            .to_str()
            .expect("Handle this better...")
            .to_string();

        let fut = self.service.call(req);

        Box::pin(async move {
            let verified = keyset.verify(&auth_header, &verifier).await;

            if let Err(e) = verified {
                error!("Token verification failed");
                return Err(ErrorNetworkAuthenticationRequired(e));
            }

            info!("Token verified successfully");

            let res = fut.await?;

            Ok(res)
        })
    }
}
