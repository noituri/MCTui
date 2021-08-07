use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;

#[derive(Debug, Serialize, Deserialize)]
pub struct Settings {
    pub auth: Auth,
    pub profiles: Profiles,
}

impl Settings {
    pub fn save(&self) {
        serde_json::to_writer_pretty(
            &File::create(format!(
                "{}/mctui.json",
                std::env::var("DOT_MCTUI").unwrap()
            ))
            .unwrap(),
            self,
        )
        .unwrap();
    }
}

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

#[derive(Debug, Serialize, Deserialize)]
pub struct Profile {
    pub id: String,
    pub name: String,
    pub version: String,
    pub asset: String,
    pub args: String,
}

impl Settings {
    pub fn new() -> Result<Self, Box<dyn Error>> {
        let settings_path = format!("{}/mctui.json", std::env::var("DOT_MCTUI").unwrap());

        if !Path::new(&settings_path).exists() {
            let file_bytes = include_bytes!("../../assets/defaultconfig.json");
            let mut file = std::fs::File::create(&settings_path)?;
            file.write_all(file_bytes)?;
            file.flush().unwrap();
        }

        let mut file = File::open(&settings_path)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;

        Ok(serde_json::from_str(&contents).unwrap())
    }
}
