use rocket::{Build, Rocket, Config as RocketConfig, routes};
use crate::server::fairings::CorsFairing;
use crate::server::add_entry::add_entry;
use crate::structures::entry::{Entry, Metadata};
use crate::structures::config::Config;
use crate::structures::users::Users;

mod config;
mod path;
mod users;
mod saving;
mod server;
mod structures;

pub const CONFIG_PATH: &str = "./config/Config.toml";
pub const USERS_PATH: &str = "./config/Users.toml";
pub const SAVE_PATH: &str = "./data";

pub struct RocketState {
    pub config: Config,
    pub users: Users,
}

#[rocket::launch]
fn launch() -> Rocket<Build> {
    let config = Config::load(CONFIG_PATH).expect("Couldn't load config");
    let users = Users::load(USERS_PATH).expect("Couldn't load users");

    let cfg = RocketConfig {
        port: config.port.unwrap_or(8080),
        ..RocketConfig::default()
    };

    rocket::custom(&cfg)
        .attach(CorsFairing {
            // FIXME: No need to read this two times: derive Clone? Rc?
            config: Config::load(CONFIG_PATH).expect("Couldn't load config"),
        })
        .mount("/api",routes![add_entry])
        .manage(RocketState {
            config,
            users,
        })
}

/*fn main() {
    println!("Hello World!");

    let test = Entry {
        metadata: Metadata {
            date: 73812738921,
            duid: "djkaskdasj".to_string(),
            page: None,
            uid: None
        },
        data: Default::default()
    };
    println!("save document - {:?}", test.save("./owo.bson").unwrap());
    println!("load document - {:?}", Entry::load("./owo.bson").unwrap());

    println!("config - {:?}", Config::load("./config/Config.toml").unwrap());

    let users = Users::load("./config/Users.toml").unwrap();
    println!("users - {:?}", users);
    println!("example user - {:?}", users.get_userdata("admin"));
}*/
