use serde::{Serialize, Deserialize};

pub mod add_entry_error;

#[derive(Serialize, Deserialize)]
pub struct GenericError {
    pub id: String,
    pub message: String,
}
