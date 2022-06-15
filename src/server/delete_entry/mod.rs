use std::fs;
use rocket::serde::json::Json;
use serde::{Serialize, Deserialize};
use crate::{Entry, SAVE_PATH};
use crate::server::guards::ProtectedApiWriteScope;

#[derive(Serialize, Deserialize)]
pub struct DeleteEntryData {
    duid: String
}

#[rocket::post("/delete", data = "<data>")]
pub fn delete_entry(data: Json<DeleteEntryData>, _protected: ProtectedApiWriteScope) -> Result<(), String> {
    let path = fs::read_dir(SAVE_PATH).unwrap();

    for entry in path {
        let path = entry.unwrap().path();

        if path.to_str().unwrap().ends_with(".bson") {
            let entry = Entry::load(&path).unwrap();

            if entry.metadata.duid == data.duid {
                fs::remove_file(path).unwrap();
            }
        }
    }

    Ok(())
}
