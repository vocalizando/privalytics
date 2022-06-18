use std::error::Error;
use std::io::Cursor;
use rocket::http::Status;
use rocket::{Request, Response};
use rocket::response::Responder;
use crate::server::routes::errors::GenericError;

pub enum AddEntryError {
    NoOriginHeader,
    InvalidProtocol,
    InvalidHostname,
    ForbiddenKeys,
    Unknown(String),
}

impl From<Box<dyn Error>> for AddEntryError {
    fn from(e: Box<dyn Error>) -> Self {
        AddEntryError::Unknown(e.to_string())
    }
}

impl<'r, 'o: 'r> Responder<'r, 'o> for AddEntryError {
    fn respond_to(self, _request: &'r Request<'_>) -> rocket::response::Result<'o> {
        let (status, error) = match self {
            AddEntryError::NoOriginHeader => {
                let error = serde_json::to_string(&GenericError {
                    id: "no-origin-header".to_string(),
                    message: "No origin header was set".to_string(),
                }).unwrap();

                (Status::BadRequest, error)
            }
            AddEntryError::InvalidProtocol => {
                let error = serde_json::to_string(&GenericError {
                    id: "invalid-protocol".to_string(),
                    message: "Invalid protocol".to_string(),
                }).unwrap();

                (Status::BadRequest, error)
            },
            AddEntryError::InvalidHostname => {
                let error = serde_json::to_string(&GenericError {
                    id: "invalid-hostname".to_string(),
                    message: "Invalid hostname".to_string(),
                }).unwrap();

                (Status::BadRequest, error)
            },
            AddEntryError::ForbiddenKeys => {
                let error = serde_json::to_string(&GenericError {
                    id: "forbidden-keys".to_string(),
                    message: "Use of non-whitelisted keys".to_string(),
                }).unwrap();

                (Status::BadRequest, error)
            },
            AddEntryError::Unknown(message) => {
                let error = serde_json::to_string(&GenericError {
                    id: "unknown".to_string(),
                    message,
                }).unwrap();

                (Status::InternalServerError, error)
            }
        };

        Ok(Response::build()
            .status(status)
            .sized_body(error.len(), Cursor::new(error))
            .finalize())

    }
}
