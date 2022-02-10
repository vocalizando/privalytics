#![feature(slice_pattern)]
#![allow(clippy::format_in_format_args)]

use crate::analytics_def::AnalyticsData;
use rocket::serde::json::Json;
use rocket::{get, launch, put, routes, Build, Config, Rocket};
use std::time::{SystemTime, UNIX_EPOCH};
use uuid::Uuid;
use crate::args::{get_args, get_env};
use crate::serialization::{deserialize, serialize};
use crate::server::{fairings, routes::api::{add_entry::add_entry}};

mod analytics_def;
mod serialization;
mod file;
mod args;
mod server;

fn is_valid_key(key: &str) -> bool {
    key.trim() != get_env().unwrap().master_key.trim()
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
    // Initial check, will be replaced in the future by a "checker" function
    let _env = get_env().expect("There are invalid or missing environment variables");

    let cfg = Config {
        port: args.port,
        ..Config::debug_default()
    };

    rocket::custom(&cfg)
        .attach(fairings::cors_fairing::CorsFairing)
        .attach(fairings::preflight_fairing::PreflightFairing)
        .mount("/api", routes![
            add_entry,
            get_data,
        ])
}

fn get_cors_hostname(hostname: &String, protocol: &String) -> String {
    format!("{}://{}", protocol, hostname)
}
