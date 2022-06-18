use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct GenericError {
    pub id: String,
    pub message: String,
}

#[macro_export]
macro_rules! impl_from_box_dyn_error {
    ($enum:ident) => {
        impl From<Box<dyn std::error::Error>> for $enum {
            fn from(e: Box<dyn std::error::Error>) -> Self {
                $enum::Unknown(e.to_string())
            }
        }
    }
}

//FIXME: Trailing comma is _not_ optional
#[macro_export]
macro_rules! match_error {
    ($status:expr, $id:literal, $message:literal,$(,)*) => {
        ($status, serde_json::to_string(&$crate::server::routes::errors::GenericError {
            id: $id.to_string(),
            message: $message.to_string(),
        }).unwrap())
    }
}

#[macro_export]
macro_rules! unknown_error {
    ($message:ident) => {

        (Status::InternalServerError, serde_json::to_string(&$crate::server::routes::errors::GenericError {
            id: "unknown".to_string(),
            message: $message,
        }).unwrap())
    }
}

#[macro_export]
macro_rules! build_response {
    ($status:ident, $error:ident) => {
        Response::build()
            .status($status)
            .sized_body($error.len(), std::io::Cursor::new($error))
            .finalize()
    }
}
