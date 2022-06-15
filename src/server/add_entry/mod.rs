#![allow(clippy::search_is_some)]

use std::mem::size_of_val;
use std::time::SystemTime;
use rocket::serde::json::Json;
use rocket::State;
use uuid::Uuid;
use serde::{Serialize, Deserialize};
use crate::{Entry, Metadata, RocketState, SAVE_PATH};
use crate::path::HashPath;
use crate::server::guards::HeadersGuard;
use crate::structures::entry::Data;

const EMPTY_STR: [&str; 3] = ["", "null", "undefined"];
const VALID_PROTOCOLS: [&str; 2] = ["http://", "https://"];

// FIXME: Private-leakage error if not ``pub`` -> try to resolve this without using ``pub``
#[derive(Serialize, Deserialize, Debug)]
pub struct RequestEntry {
    metadata: RequestMetadata,
    data: Data,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RequestMetadata {
    pub date: Option<u64>,
    pub duid: Option<String>,
    pub page: Option<String>,
    pub uid: Option<String>,
}

impl From<RequestEntry> for Entry {
    fn from(e: RequestEntry) -> Self {
        Entry {
            metadata: Metadata {
                date: e.metadata.date.expect("Conversion from RequestEntry to Entry failed -> missing date"),
                duid: e.metadata.duid.expect("Conversion from RequestEntry to Entry failed -> missing duid"),
                page: e.metadata.page,
                uid: e.metadata.uid,
            },
            data: e.data
        }
    }
}

// TODO: Add Authorization guard
#[rocket::post("/submit", data = "<data>")]
pub fn add_entry(data: Json<RequestEntry>, headers_guard: HeadersGuard, _state: &State<RocketState>) -> Result<(), String> {
    let headers = headers_guard.headers;
    let mut data = data.into_inner();
    data.metadata.date = Some(SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .expect("Clock is going backwards?")
        .as_millis() as u64);

    data.metadata.duid = Some(Uuid::new_v4().to_string());

    // TODO: Implement server-side UID and page support

    let data = data;
    let origin_header = headers.get_one("Origin").unwrap_or("null");

    if EMPTY_STR.contains(&origin_header) {
        return Err(String::from("Null Origin header"))
    }
    if VALID_PROTOCOLS.iter()
        .find(|v| origin_header.starts_with(*v))
        .is_none() {
        return Err(String::from("Invalid protocol"))
    }

    let split_header = origin_header.split("://").collect::<Vec<&str>>();
    if split_header.len() < 2 {
        return Err(String::from("Invalid hostname"))
    }

    let data = Entry::from(data);
    let filename = format!("{}/{}.bson", SAVE_PATH, &data.metadata.duid);

    if let Err(e) = data.save(&filename) {
        return Err(e.to_string())
    }

    Ok(())
}
