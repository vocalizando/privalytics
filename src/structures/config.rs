use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub client_key: Option<String>,
    pub port: Option<u16>,
}
