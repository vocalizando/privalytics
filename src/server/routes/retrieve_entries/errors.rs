use rocket::http::Status;
use rocket::{Request, Response};
use rocket::response::Responder;
use crate::{build_response, impl_from_box_dyn_error, match_error, unknown_error};

pub enum RetrieveEntriesError {
    InvalidFromOrTo,
    Unknown(String),
}

impl_from_box_dyn_error!(RetrieveEntriesError);

impl<'r, 'o: 'r> Responder<'r, 'o> for RetrieveEntriesError {
    fn respond_to(self, _request: &'r Request<'_>) -> rocket::response::Result<'o> {
        let (status, error) = match self {
            RetrieveEntriesError::InvalidFromOrTo => {
                match_error! {
                    Status::BadRequest,
                    "invalid-from-or-to",
                    "Invalid ``from`` or ``to`` values",
                }
            },
            RetrieveEntriesError::Unknown(message) => {
                unknown_error!(message)
            },
        };

        Ok(build_response!(status, error))
    }
}
