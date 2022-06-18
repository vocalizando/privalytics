use std::error::Error;
use std::io::Cursor;
use rocket::http::Status;
use rocket::{Request, Response};
use rocket::response::Responder;
use crate::server::routes::errors::GenericError;

pub enum DeleteEntryError {
    NotFound,
    ResourceBusy,
    Unknown(String),
}

impl From<Box<dyn Error>> for DeleteEntryError {
    fn from(e: Box<dyn Error>) -> Self {
        DeleteEntryError::Unknown(e.to_string())
    }
}

impl<'r, 'o: 'r> Responder<'r, 'o> for DeleteEntryError {
    fn respond_to(self, _request: &'r Request<'_>) -> rocket::response::Result<'o> {
        let (status, error) = match self {
            DeleteEntryError::NotFound => {
                let error = serde_json::to_string(&GenericError {
                    id: "not-found".to_string(),
                    message: "entry not found".to_string(),
                }).unwrap();

                (Status::NotFound, error)
            },
            DeleteEntryError::ResourceBusy => {
                let error = serde_json::to_string(&GenericError {
                    id: "not-available".to_string(),
                    message: "entry file is not available".to_string(),
                }).unwrap();

                (Status::InternalServerError, error)
            },
            DeleteEntryError::Unknown(message) => {
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
