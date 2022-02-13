use crate::auth::defs::responses::server_responses::{LoginCode, LoginEndpointResponse};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginEndpointIntermediateResponse {
    pub code: i8,
    pub jwt: Option<String>,
}

impl LoginEndpointIntermediateResponse {
    pub fn to_final_response(&self) -> LoginEndpointResponse {
        LoginEndpointResponse {
            code: LoginCode::from(self.code),
            jwt: self.jwt.to_owned(),
        }
    }
}
