use crate::auth::defs::responses::login_error::LoginError;
use crate::auth::defs::responses::server_responses::TokenClaims;
use crate::auth::requester::{Requester, RequesterTrait};
use rocket::request::{FromRequest, Outcome};
use rocket::Request;

pub struct AuthData {
    pub data: TokenClaims,
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for AuthData {
    type Error = LoginError;

    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let identifier = request
            .headers()
            .get_one("X-Privalytics-Identifier")
            .unwrap();
        let password = request.headers().get_one("X-Privalytics-Password").unwrap();

        let requester = Requester::from_endpoint("http://127.0.0.1:8080/api/login");

        let data = requester.get_token(identifier, password).await.unwrap();

        Outcome::Success(AuthData { data })
    }
}
