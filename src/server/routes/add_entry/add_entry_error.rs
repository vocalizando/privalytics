use rocket::http::Status;
use rocket::{Request, Response};
use rocket::response::Responder;
use crate::{build_response, impl_from_box_dyn_error, match_error, unknown_error};

pub enum AddEntryError {
    NoOriginHeader,
    InvalidProtocol,
    InvalidHostname,
    ForbiddenKeys,
    Unknown(String),
}

impl_from_box_dyn_error!(AddEntryError);

impl<'r, 'o: 'r> Responder<'r, 'o> for AddEntryError {
    fn respond_to(self, _request: &'r Request<'_>) -> rocket::response::Result<'o> {
        let (status, error) = match self {
            AddEntryError::NoOriginHeader => {
                match_error! {
                    Status::BadRequest,
                    "no-origin-header",
                    "No origin header was set",
                }
            }
            AddEntryError::InvalidProtocol => {
                match_error! {
                    Status::BadRequest,
                    "invalid-protocol",
                    "Invalid protocol",
                }
            },
            AddEntryError::InvalidHostname => {
                match_error! {
                    Status::BadRequest,
                    "invalid-hostname",
                    "Invalid hostname",
                }
            },
            AddEntryError::ForbiddenKeys => {
                match_error! {
                    Status::BadRequest,
                    "forbidden-keys",
                    "Use of non-whitelisted keys",
                }
            },
            AddEntryError::Unknown(message) => {
                unknown_error!(message)
            }
        };

        Ok(build_response!(status, error))

    }
}
