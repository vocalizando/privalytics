use crate::auth::defs::scope::Scope;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct TokenClaims {
    pub exp: usize,
    pub name: String,
    pub scopes: Vec<Scope>,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum LoginCode {
    ServerError = -3,
    InvalidPassword = -2,
    InvalidIdentifier = -1,
    Ok = 0,
}

impl From<i8> for LoginCode {
    fn from(value: i8) -> Self {
        match value {
            -3 => LoginCode::ServerError,
            -2 => LoginCode::InvalidPassword,
            -1 => LoginCode::InvalidIdentifier,
            0 => LoginCode::Ok,
            _ => LoginCode::ServerError,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginEndpointResponse {
    pub code: LoginCode,
    pub jwt: Option<String>,
}
