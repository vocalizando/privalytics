use crate::auth::defs::responses::server_responses::LoginCode;

#[derive(Debug)]
pub struct LoginError {
    pub code: LoginCode,
}

impl LoginError {
    pub fn from_login_code(login_code: LoginCode) -> LoginError {
        LoginError { code: login_code }
    }
}

impl TryFrom<reqwest::Error> for LoginError {
    type Error = LoginError;

    fn try_from(_value: reqwest::Error) -> Result<Self, Self::Error> {
        Ok(Self::Error {
            code: LoginCode::ServerError,
        })
    }
}
