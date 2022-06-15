use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Users {
    pub users: Vec<UserData>
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct UserData {
    pub username: String,
    pub token: String,
    pub scope: Scope,
}

#[derive(Serialize, Deserialize, PartialEq, PartialOrd, Clone, Debug)]
pub enum Scope {
    Read = 0,
    Write = 1,
    Admin = 2,
}
