use std::fs;
use rocket::serde::json::Json;
use serde::{Serialize, Deserialize};
use crate::{SAVE_PATH};
use crate::server::guards::ProtectedApiWriteScope;

#[derive(Serialize, Deserialize)]
pub struct DeleteEntryData {
    duid: String
}

#[rocket::post("/delete", data = "<data>")]
pub fn delete_entry(data: Json<DeleteEntryData>, _protected: ProtectedApiWriteScope) -> Result<(), String> {
    // FIXME: Check no ``/`` or ``\`` are included on the requested duid

    return if let Err(e) = fs::remove_file(format!("{}/{}.bson", SAVE_PATH, data.duid)) {
        Err(e.into_inner().unwrap().to_string())
    } else {
        Ok(())
    };
}

#[rocket::options("/delete")]
pub fn delete_entry_options() -> Result<(), ()> {
    Ok(())
}
