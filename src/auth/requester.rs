use async_trait::async_trait;
use jsonwebtoken::{DecodingKey, Validation};
use serde::{Deserialize, Serialize};
use crate::auth::defs::responses::{LoginCode, LoginEndpointResponse, TokenClaims};
use crate::auth::defs::responses::LoginCode::ServerError;

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

        let intermediate_resp: LoginEndpointIntermediateResponse = serde_json::from_str::<LoginEndpointIntermediateResponse>(
            resp_text.as_str()
        ).unwrap();

        let resp = intermediate_resp.to_final_response();

        match resp.code {
            LoginCode::Ok => Ok(decode_jwt(identifier, password, &resp.jwt.unwrap())),
            _ => Err(LoginError::from_login_code(resp.code))
        }
    }
}

fn decode_jwt(identifier: &str, password: &str, jwt: &String) -> TokenClaims {
    println!("{}", &jwt);
    jsonwebtoken::decode::<TokenClaims>(
        &jwt.as_str(),
        &DecodingKey::from_secret(format!("{}{}", identifier, password).as_ref()),
        &Validation::default(),
    ).unwrap().claims
}

#[derive(Debug, Serialize, Deserialize)]
struct LoginEndpointIntermediateResponse {
    pub code: i8,
    pub jwt: Option<String>,
}

impl LoginEndpointIntermediateResponse {
    fn to_final_response(&self) -> LoginEndpointResponse {
        LoginEndpointResponse {
            code: LoginCode::from(*&self.code),
            jwt: (&self.jwt).to_owned(),
        }
    }
}

#[derive(Debug)]
pub struct LoginError {
    code: LoginCode,
}

impl LoginError {
    pub fn from_login_code(login_code: LoginCode) -> LoginError {
        LoginError {
            code: login_code,
        }
    }
}

impl TryFrom<reqwest::Error> for LoginError {
    type Error = LoginError;

    fn try_from(value: reqwest::Error) -> Result<Self, Self::Error> {
        Ok(Self::Error {
            code: LoginCode::ServerError,
        })
    }
}
