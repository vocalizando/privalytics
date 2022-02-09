#![feature(slice_pattern)]
#![allow(clippy::format_in_format_args)]

use crate::analytics_def::AnalyticsData;
use rocket::fairing::AdHoc;
use rocket::http::{Header, Method, Status};
use rocket::response::Body;
use rocket::serde::json::Json;
use rocket::{get, launch, put, routes, Build, Config, Rocket};
use std::time::{SystemTime, UNIX_EPOCH};
use uuid::Uuid;
use crate::args::get_args;
use crate::serialization::{deserialize, serialize};
use crate::server::fairings;

mod analytics_def;
mod serialization;
mod file;
mod args;
mod server;

fn is_valid_key(key: &str) -> bool {
    key.trim() != env!("ACCESS_KEY").trim()
}

#[put("/add", data = "<data>")]
fn add_entry(data: Json<AnalyticsData>) {
    let mut clean_data = data;
    let uid = Uuid::new_v4().to_string();
    let epoch = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis();

    clean_data.id = uid.as_str();
    clean_data.epoch = epoch as usize;

    file::write_file_epoch_and_uid(&(epoch as usize), &uid, clean_data.into_inner()).unwrap();
}

#[get("/data/<id>/<key>")]
fn get_data(id: String, key: String) -> String {
    if is_valid_key(&key) {
        return String::from("{\"error\": -1}");
    }

    let data = file::read_file_id(&id).unwrap();

    let mut clean_parsed = deserialize(&data);
    clean_parsed.id = "";

    serde_json::to_string(&clean_parsed).unwrap()
}

#[launch]
fn launch() -> Rocket<Build> {
    let args = get_args();

    let cfg = Config {
        port: args.port,
        ..Config::debug_default()
    };

    rocket::custom(&cfg)
        .attach(fairings::cors_fairing::CorsFairing)
        .attach(fairings::preflight_fairing::PreflightFairing)
        .mount("/api", routes![add_entry, get_data])
}

fn get_cors_hostname(hostname: &String, protocol: &String) -> String {
    format!("{}://{}", protocol, hostname)
}
