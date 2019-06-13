use crate::constants::DOT_MCTUI;
use serde::{Deserialize, Serialize};
use std::path::Path;
use std::io::{Write, Read};
use std::fs::{File, create_dir_all};
use std::error::Error;

#[derive(Debug, Serialize, Deserialize)]
pub struct Settings {
    pub auth: Auth,
    pub profiles: Profiles
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Auth {
    pub username: String,
    pub access_token: String,
    pub client_token: String,
    pub online: bool
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Profiles {
    pub selected: String,
    pub profiles: Vec<Profile>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Profile {
    pub id: String,
    pub name: String,
    pub version: String,
    pub asset: String
}

impl Settings {
    pub fn new() -> Result<Self, Box<dyn Error>> {
        let settings_path = format!("{}/mctui.json", DOT_MCTUI);

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

impl Profiles {
    pub fn get_profile(&self, id: &str) -> Option<&Profile> {
        for p in &self.profiles {
            if p.id == id {
                return Some(p);
            }
        }

        None
    }

    pub fn set_profile(&mut self, id: String) -> Result<(),()> {
        for p in &self.profiles {
            if p.id == id {
                self.selected = id;
                serde_json::to_writer_pretty(&File::create(format!("{}/mctui.json", DOT_MCTUI)).unwrap(),self).unwrap();
                return Ok(());
            }
        }

        Err(())
    }
}