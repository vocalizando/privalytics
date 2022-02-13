use std::time::{SystemTime, UNIX_EPOCH};
use jsonwebtoken::{EncodingKey, Header};
use serde::{Serialize, Deserialize};
use rocket::{Build, Config, Rocket, routes, post, launch};
use rocket::serde::json::Json;
use serde_json::json;

fn print_warning() {
    println!("=== WARNING ===");
    println!("You MUST NOT use this server in production");
    println!("This server is purely to help the development of the library");
    println!("You may check the implementation, but with a pinch of salt");
    println!("This ain't Enterprisey Ready Verfiiedd®");
    println!("=== WARNING ===");
}

#[derive(Debug, Serialize, Deserialize)]
struct LoginReq {
    identifier: String,
    password: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct LoginJWT {
    pub exp: usize,
    pub name: String,
    pub scopes: Vec<u16>,
}

#[post("/login", data = "<data>")]
fn login(data: Json<LoginReq>) -> String {
    let token = jsonwebtoken::encode(
        &Header::default(),
        &LoginJWT {
            exp: usize::MAX,
            name: String::from("Random name"),
            scopes: vec![1, 2, 3, 4, 5],
        },
        &EncodingKey::from_secret(format!("{}{}", data.identifier, data.password).as_ref())
    ).unwrap();

    let data = serde_json::to_string(&json!({
        "code": 0,
        "jwt": token,
    })).unwrap();

    println!("{}", data);

    data
}

#[launch]
fn launch() -> Rocket<Build> {
    print_warning();

    let cfg = Config {
        port: 8080,
        ..Config::debug_default()
    };

    rocket::custom(&cfg)
        .mount("/api", routes![login])
}
