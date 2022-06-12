use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Users {
    pub users: Vec<UserData>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UserData {
    pub username: String,
    pub token: String,
    pub scope: Scope,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Scope {
    Read,
    Write,
    Admin,
}
