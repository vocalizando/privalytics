use crate::auth::defs::responses::intermediate::LoginEndpointIntermediateResponse;
use crate::auth::defs::responses::login_error::LoginError;
use crate::auth::defs::responses::server_responses::{LoginCode, TokenClaims};
use async_trait::async_trait;
use jsonwebtoken::{DecodingKey, Validation};

pub struct Requester {
    endpoint: String,
}

impl Requester {
    pub fn from_endpoint(endpoint: &str) -> Requester {
        Requester {
            endpoint: String::from(endpoint),
        }
    }
}

#[async_trait]
pub trait RequesterTrait {
    async fn get_token(&self, identifier: &str, password: &str) -> Result<TokenClaims, LoginError>;
}

#[async_trait]
impl RequesterTrait for Requester {
    async fn get_token(&self, identifier: &str, password: &str) -> Result<TokenClaims, LoginError> {
        let resp_text = reqwest::Client::new()
            .post(&self.endpoint)
            .json(&serde_json::json!({
                "identifier": identifier,
                "password": password,
            }))
            .send()
            .await
            .unwrap()
            .text()
            .await
            .unwrap();

        let intermediate_resp: LoginEndpointIntermediateResponse =
            serde_json::from_str::<LoginEndpointIntermediateResponse>(resp_text.as_str()).unwrap();

        let resp = intermediate_resp.to_final_response();

        match resp.code {
            LoginCode::Ok => Ok(decode_jwt(identifier, password, &resp.jwt.unwrap()).unwrap()),
            _ => Err(LoginError::from_login_code(resp.code)),
        }
    }
}

fn decode_jwt(identifier: &str, password: &str, jwt: &str) -> Result<TokenClaims, ()> {
    let unchecked_claims: TokenClaims = jsonwebtoken::decode::<TokenClaims>(
        jwt,
        &DecodingKey::from_secret(format!("{}{}", identifier, password).as_ref()),
        &Validation::default(),
    )
    .unwrap()
    .claims;

    for scope in &unchecked_claims.scopes {
        if !scope.is_valid(&unchecked_claims.scopes) {
            return Err(());
        }
    }

    Ok(unchecked_claims)
}
