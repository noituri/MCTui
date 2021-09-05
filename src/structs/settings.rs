use serde::{Deserialize, Serialize};

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
