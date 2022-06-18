use std::error::Error;
use std::io::Cursor;
use rocket::http::Status;
use rocket::{Request, Response};
use rocket::response::Responder;
use crate::server::routes::errors::GenericError;

pub enum RetrieveEntriesError {
    Unknown(String),
}

impl From<Box<dyn Error>> for RetrieveEntriesError {
    fn from(e: Box<dyn Error>) -> Self {
        RetrieveEntriesError::Unknown(e.to_string())
    }
}

impl<'r, 'o: 'r> Responder<'r, 'o> for RetrieveEntriesError {
    fn respond_to(self, _request: &'r Request<'_>) -> rocket::response::Result<'o> {
        let (status, error) = match self {
            RetrieveEntriesError::Unknown(message) => {
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
