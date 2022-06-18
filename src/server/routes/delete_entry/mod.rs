use std::fs;
use std::io::ErrorKind;
use rocket::serde::json::Json;
use serde::{Serialize, Deserialize};
use crate::{SAVE_PATH};
use crate::server::guards::ProtectedApiWriteScope;
use crate::server::routes::delete_entry::errors::DeleteEntryError;

mod errors;

#[derive(Serialize, Deserialize)]
pub struct DeleteEntryData {
    duid: String
}

#[rocket::post("/delete", data = "<data>")]
pub fn delete_entry(data: Json<DeleteEntryData>, _protected: ProtectedApiWriteScope) -> Result<(), DeleteEntryError> {
    // FIXME: Check no ``/`` or ``\`` are included on the requested duid

    return if let Err(e) = fs::remove_file(format!("{}/{}.bson", SAVE_PATH, data.duid)) {
        match e.kind() {
            ErrorKind::NotFound => Err(DeleteEntryError::NotFound),
            ErrorKind::ResourceBusy => Err(DeleteEntryError::ResourceBusy),
            _ => {
                Err(DeleteEntryError::Unknown(
                    e.into_inner()
                        .ok_or_else(|| DeleteEntryError::Unknown("into_inner failed".to_string()))?
                        .to_string()
                ))
            }
        }
    } else {
        Ok(())
    }
}

#[rocket::options("/delete")]
pub fn delete_entry_options() -> Result<(), ()> {
    Ok(())
}
