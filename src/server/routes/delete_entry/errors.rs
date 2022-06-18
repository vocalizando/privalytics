use rocket::http::Status;
use rocket::{Request, Response};
use rocket::response::Responder;
use crate::{build_response, impl_from_box_dyn_error, match_error, unknown_error};

pub enum DeleteEntryError {
    NotFound,
    ResourceBusy,
    Unknown(String),
}

impl_from_box_dyn_error!(DeleteEntryError);

impl<'r, 'o: 'r> Responder<'r, 'o> for DeleteEntryError {
    fn respond_to(self, _request: &'r Request<'_>) -> rocket::response::Result<'o> {
        let (status, error) = match self {
            DeleteEntryError::NotFound => {
                match_error! {
                    Status::NotFound,
                    "not-found",
                    "Entry not found",
                }
            },
            DeleteEntryError::ResourceBusy => {
                match_error! {
                    Status::InternalServerError,
                    "not-available",
                    "Entry file is not available",
                }
            },
            DeleteEntryError::Unknown(message) => {
                unknown_error!(message)
            }
        };

        Ok(build_response!(status, error))
    }
}
