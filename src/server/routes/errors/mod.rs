use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct GenericError {
    pub id: String,
    pub message: String,
}
