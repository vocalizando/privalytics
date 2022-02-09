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

mod analytics_def;
mod serialization;
mod file;
mod args;

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
        .attach(AdHoc::on_response("Add CORS", move |_, res| {
            let args = get_args();
            Box::pin(async move {
                res.set_header(Header::new(
                    "Access-Control-Allow-Origin",
                    get_cors_hostname(&args.cors_hostname, &args.cors_protocol),
                ));
            })
        }))
        .attach(AdHoc::on_response("Add Preflight", move |req, res| {
            let args = get_args();
            Box::pin(async move {
                if req.method() == Method::Options {
                    let empty_body = Body::default();

                    res.set_status(Status::NoContent);
                    res.set_header(Header::new(
                        "Access-Control-Allow-Origin",
                        get_cors_hostname(&args.cors_hostname, &args.cors_protocol),
                    ));
                    res.set_header(Header::new(
                        "Access-Control-Allow-Methods",
                        "PUT, POST, GET, OPTIONS, DELETE",
                    ));
                    res.set_header(Header::new("Access-Control-Allow-Headers", "Content-Type"));
                    res.set_header(Header::new("Access-Control-Max-Age", "86400"));
                    res.set_streamed_body(empty_body);
                }
            })
        }))
        .mount("/api", routes![add_entry, get_data])
}

fn get_cors_hostname(hostname: &String, protocol: &String) -> String {
    format!("{}{}", protocol, hostname)
}
