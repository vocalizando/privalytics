use std::error::Error;
use std::fs;
use std::path::Path;
use crate::structures::config::Config;

impl Config {
    pub fn load<P: AsRef<Path> + ?Sized>(path: &P) -> Result<Config, Box<dyn Error>> {
        let raw = fs::read_to_string(path)?;
        let value: Config = toml::from_str(raw.as_str())?;

        Ok(value)
    }
}
