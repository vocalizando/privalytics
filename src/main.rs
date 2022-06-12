use crate::structures::analytics::{Entry, Metadata};
use crate::structures::config::Config;

mod config;
mod saving;
mod structures;

fn main() {
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
}
