use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Auth {
    pub username: String,
    pub access_token: String,
    pub client_token: String,
    pub online: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Profiles {
    pub selected: String,
    pub profiles: Vec<Profile>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Profile {
    pub id: String,
    pub name: String,
    pub version: String,
    pub asset: String,
    pub args: String,
}
