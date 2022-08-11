use actix_web::post;
use actix_web::web::{Form, Json};
use log::{error, info};
use oauth2::basic::{BasicClient, BasicTokenType};
use oauth2::reqwest::async_http_client;
use oauth2::{
    AuthUrl, AuthorizationCode, ClientId, ClientSecret, EmptyExtraTokenFields, PkceCodeVerifier,
    RedirectUrl, StandardTokenResponse, TokenUrl,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct TokenBody {
    grant_type: String,
    code: String,
    code_verifier: String,
    redirect_uri: String,
}

pub type Oauth2StandardTokenResponse = StandardTokenResponse<EmptyExtraTokenFields, BasicTokenType>;

const CLIENT_ID: &str = "2k4mvc8v79o3b1vnvvmtgl36g6";
const CLIENT_SECRET: &str = "698253smt70nn5lak2kdea1gabllkc5shj0nueuf4gnuk302564";
const TOKEN_URL: &str = "https://listr.auth.us-east-1.amazoncognito.com/oauth2/token";

#[post("/token")]
pub async fn token(body: Form<TokenBody>) -> Json<Oauth2StandardTokenResponse> {
    let req = body.into_inner();

    let client = BasicClient::new(
        ClientId::new(CLIENT_ID.to_string()),
        Some(ClientSecret::new(CLIENT_SECRET.to_string())),
        AuthUrl::new("http://auth".to_string()).expect("Failed to create AuthUrl"), //Not used. Configured by AWS Cognito
        Some(TokenUrl::new(TOKEN_URL.to_string()).expect("Failed to create TokenUrl")),
    )
    .set_redirect_uri(RedirectUrl::new(req.redirect_uri).unwrap());

    let pkce_verifier = PkceCodeVerifier::new(req.code_verifier);
    let token_result = client
        .exchange_code(AuthorizationCode::new(req.code))
        .set_pkce_verifier(pkce_verifier)
        .request_async(async_http_client)
        .await;

    match token_result {
        Ok(val) => {
            info!("Tokens received from OAuth provider.");
            Json(val)
        }
        Err(e) => {
            error!("{:?}", e.to_string());
            panic!("Handle this better")
        }
    }
}
