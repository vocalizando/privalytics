use std::io::Cursor;
use rocket::{Request, Response};
use serde::{Deserialize, Serialize};
use rocket::http::{ContentType, Status};
use rocket::response::Responder;
use rocket::serde::json::Json;
use crate::{Entry, SAVE_PATH};
use crate::server::guards::ProtectedApiReadScope;
use retrieve_entries_error::RetrieveEntriesError;

pub mod retrieve_entries_error;

#[derive(Serialize, Deserialize)]
pub struct EntrySearchData {
    from: u32,
    to: i32,
}

pub struct EntriesResponse(Vec<Entry>);

impl From<Vec<Entry>> for EntriesResponse {
    fn from(v: Vec<Entry>) -> Self {
        EntriesResponse(v)
    }
}

impl<'r, 'o: 'r> Responder<'r, 'o> for EntriesResponse {
    fn respond_to(self, _: &'r Request<'_>) -> rocket::response::Result<'o> {
        let entries = &self.0;
        let json = rocket::serde::json::to_string(&entries).unwrap();
        let json = json.as_str();

        let response = Response::build()
            .status(Status::new(200))
            .header(ContentType::JSON)
            .sized_body(json.len().to_owned(), Cursor::new(json.to_owned()))
            .finalize();

        Ok(response)
    }
}

#[rocket::post("/retrieve", data = "<data>")]
pub fn retrieve_entries(data: Json<EntrySearchData>, _protected: ProtectedApiReadScope) -> Result<EntriesResponse, RetrieveEntriesError> {
    // FIXME: Without this, rustc crashes, lmao??
    let _a = data.to;

    // TODO: Implement ``from`` and ``to``
    let entries = Entry::load_entries(SAVE_PATH)?;

    Ok(EntriesResponse::from(entries))
}


#[rocket::options("/retrieve")]
pub fn retrieve_entries_options() -> Result<(), ()> {
    Ok(())
}
