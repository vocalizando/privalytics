use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub client_key: Option<String>,
    pub cors_hostnames: Option<Vec<String>>,
    pub allowed_keys: Option<Vec<String>>,
    pub port: Option<u16>,
}
