use std::error::Error;
use rocket::http::HeaderMap;
use rocket::Request;
use rocket::request::{FromRequest, Outcome};

pub struct HeadersGuard<'r> {
    pub headers: HeaderMap<'r>
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for HeadersGuard<'r> {
    type Error = Box<dyn Error>;

    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let headers = request.headers();
        let headers_clone = headers.clone().to_owned();
        Outcome::Success(HeadersGuard {
            headers: headers_clone,
        })
    }
}
