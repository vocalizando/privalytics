#![allow(clippy::search_is_some)]

use std::time::SystemTime;
use rocket::serde::json::Json;
use rocket::State;
use uuid::Uuid;
use serde::{Serialize, Deserialize};
use crate::{Entry, Metadata, RocketState, SAVE_PATH};
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
#[rocket::post("/submit", data = "<entry>")]
pub fn add_entry(entry: Json<RequestEntry>, headers_guard: HeadersGuard, state: &State<RocketState>) -> Result<(), String> {
    let headers = headers_guard.headers;
    let mut entry = entry.into_inner();
    entry.metadata.date = Some(SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .expect("Clock is going backwards?")
        .as_millis() as u64);

    entry.metadata.duid = Some(Uuid::new_v4().to_string());

    // TODO: Implement server-side UID and page support

    let entry = entry;
    let origin_header = headers.get_one("Origin").unwrap_or("null");

    if EMPTY_STR.contains(&origin_header) {
        return Err(String::from("Null Origin header"))
    }
    if VALID_PROTOCOLS.iter()
        .find(|v| origin_header.starts_with(*v))
        .is_none() {
        return Err(String::from("Invalid protocol"))
    }

    if origin_header.split("://").count() < 2 {
        return Err(String::from("Invalid hostname"))
    }

    if let Some(allowed_keys) = &state.config.allowed_keys {
        for (key, _v) in &entry.data {
            if !allowed_keys.contains(key) {
                return Err(String::from("Use of invalid keys"))
            }
        }
    }

    let entry = Entry::from(entry);
    let filename = format!("{}/{}.bson", SAVE_PATH, &entry.metadata.duid);

    if let Err(e) = entry.save(&filename) {
        return Err(e.to_string())
    }

    Ok(())
}

#[rocket::options("/submit")]
pub fn add_entry_options() -> Result<(), ()> {
    Ok(())
}
