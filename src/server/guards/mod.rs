use std::error::Error;
use rocket::http::{HeaderMap, Status};
use rocket::{Request, State};
use rocket::request::{FromRequest, Outcome};
use crate::RocketState;
use crate::structures::users::Scope;

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

pub struct ProtectedApiReadScope;
pub struct ProtectedApiWriteScope;
pub struct ProtectedApiAdminScope;

macro_rules! api_scope {
    ($struct:ident, $enum_value:expr) => {
        #[rocket::async_trait]
        impl<'r> FromRequest<'r> for $struct {
            type Error = String;

            async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
                let headers = request.headers();

                if let Some(header) = headers.get_one("Authorization") {
                    let user = header.split_whitespace().collect::<Vec<&str>>().get(1).unwrap().split(':').collect::<Vec<&str>>();
                    let username = user.first().unwrap();
                    let token = user.get(1).unwrap();
                    let state = request.guard::<&State<RocketState>>().await.unwrap();

                    if let Some(userdata) = state.users.get_userdata(username) {
                        if userdata.token == **token && userdata.scope >= $enum_value {
                            Outcome::Success($struct)
                        } else {
                            Outcome::Failure((
                                Status::new(401),
                                "Invalid user".to_string()
                            ))
                        }
                    } else {
                        Outcome::Failure((
                            Status::new(401),
                            "Invalid user".to_string()
                        ))
                    }


                } else {
                    Outcome::Failure((
                        Status::new(400),
                        "No Authorization header".to_string()
                    ))
                }
            }
        }
    };
}

api_scope!(ProtectedApiReadScope, Scope::Read);
api_scope!(ProtectedApiWriteScope, Scope::Write);
api_scope!(ProtectedApiAdminScope, Scope::Admin);
