#![feature(slice_pattern)]

use crate::analytics_def::DbAnalyticsData;
use bincode::{decode_from_slice, encode_to_vec};
use clap::Parser;
use rocket::fairing::AdHoc;
use rocket::http::{Header, Method, Status};
use rocket::response::Body;
use rocket::serde::json::Json;
use rocket::{get, launch, put, routes, Build, Config, Rocket};
use std::fs;
use std::fs::OpenOptions;
use std::io::Write;


use std::time::{SystemTime, UNIX_EPOCH};
use uuid::Uuid;

mod analytics_def;

fn is_valid_key(key: &str) -> bool {
    key.trim() != env!("ACCESS_KEY").trim()
}

#[put("/add", data = "<data>")]
fn add_entry(data: Json<DbAnalyticsData>) {
    let mut clean_data = data;
    let uid = Uuid::new_v4().to_string();
    let epoch = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis();

    clean_data.id = uid.as_str();
    clean_data.epoch = epoch as usize;

    println!(
        "{:?} {}",
        clean_data,
        format!("analytics-data/{}-{}.plytics.bin", &epoch, &uid)
    );

    let mut file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(format!("analytics-data/{}-{}.plytics.bin", &epoch, &uid))
        .expect("Unable to read/create/write file");

    let _ = file.write_all(
        encode_to_vec(clean_data.into_inner(), bincode::config::standard())
            .unwrap()
            .as_slice(),
    );
}

#[get("/data/<id>/<key>")]
fn get_data(id: String, key: String) -> String {
    if is_valid_key(&key) {
        return String::from("{\"error\": -1}");
    }

    let data = fs::read(format!("analytics-data/{}.plytics.bin", &id)).unwrap();
    let (parsed, _): (DbAnalyticsData, usize) =
        decode_from_slice(&data[..], bincode::config::standard()).unwrap();

    let mut clean_parsed = parsed;
    clean_parsed.id = "";

    serde_json::to_string(&clean_parsed).unwrap()
}

#[launch]
fn launch() -> Rocket<Build> {
    let args: Args = Args::parse();

    let cfg = Config {
        port: args.port,
        ..Config::debug_default()
    };

    rocket::custom(&cfg)
        .attach(AdHoc::on_response("Add CORS", move |_, res| {
            let args: Args = Args::parse();
            Box::pin(async move {
                res.set_header(Header::new(
                    "Access-Control-Allow-Origin",
                    get_cors_hostname(&args.cors_hostname, &args.cors_secure),
                ));
            })
        }))
        .attach(AdHoc::on_response("Add Preflight", move |req, res| {
            let args: Args = Args::parse();
            Box::pin(async move {
                if req.method() == Method::Options {
                    let empty_body = Body::default();

                    res.set_status(Status::NoContent);
                    res.set_header(Header::new(
                        "Access-Control-Allow-Origin",
                        get_cors_hostname(&args.cors_hostname, &args.cors_secure),
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

#[derive(Parser, Debug, Clone)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(short, long, default_value_t = 80)]
    port: u16,

    #[clap(long)]
    cors_hostname: String,

    #[clap(long)]
    cors_secure: bool,
}

fn get_cors_hostname(hostname: &String, secure: &bool) -> String {
    if *secure {
        format!("https://{}", hostname)
    } else {
        format!("http://{}", hostname)
    }
}
