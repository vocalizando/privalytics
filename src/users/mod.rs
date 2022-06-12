use std::error::Error;
use std::fs;
use std::path::Path;
use crate::structures::users::{UserData, Users};

impl Users {
    pub fn load<P: AsRef<Path> + ?Sized>(path: &P) -> Result<Users, Box<dyn Error>> {
        let raw = fs::read_to_string(path)?;
        let value: Users = toml::from_str(raw.as_str())?;

        Ok(value)
    }

    pub fn get_userdata(&self, username: &str) -> Option<&UserData> {
        let users = self.users.iter()
            .filter(|d| d.username == *username)
            .map(|d| d.clone().to_owned())
            .collect::<Vec<&UserData>>();

        if let Some(v) = users.get(0) {
            Some(*v)
        } else {
            None
        }
    }
}
