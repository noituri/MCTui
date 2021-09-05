use platform_dirs::AppDirs;
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fs::File;
use std::io::Read;
use uuid::Uuid;

const FILE_NAME: &str = "mctui.json";

#[derive(Debug, Serialize, Deserialize)]
pub struct Settings {
    pub auth: Auth,
    pub profiles: Profiles,

    /// FIXME: Temporary solution until Settings refactoring
    #[serde(skip)]
    pub app_dirs: Option<AppDirs>,
}

impl Settings {
    pub fn new(app_dirs: AppDirs) -> Result<Self, Box<dyn Error>> {
        let settings_path = app_dirs.data_dir.join(FILE_NAME);

        let mut settings = match settings_path.exists() {
            true => {
                let mut file = File::open(&settings_path)?;
                let mut contents = String::new();
                file.read_to_string(&mut contents)?;

                serde_json::from_str(&contents)?
            }
            false => Settings::default(),
        };

        // FIXME: Temporary solution until Settings refactoring
        settings.app_dirs = Some(app_dirs);

        Ok(settings)
    }

    pub fn save(&self) {
        // FIXME: Option<T> Temporary solution until Settings refactoring
        let settings_path = self.app_dirs.as_ref().unwrap().data_dir.join(FILE_NAME);

        serde_json::to_writer_pretty(&File::create(settings_path).unwrap(), self).unwrap();
    }

    pub fn get_profile(&self, id: &str) -> Option<Profile> {
        self.profiles
            .profiles
            .iter()
            .find(|x| x.id == id)
            .map(Clone::clone)
    }

    pub fn create_profile(&mut self, name: String, version: String, asset: String, args: String) {
        let mut id = Uuid::new_v4().to_string();

        loop {
            let mut exists = false;
            for p in &self.profiles.profiles {
                if p.id == id {
                    id = Uuid::new_v4().to_string();
                    exists = true
                }
            }

            if !exists {
                break;
            }
        }

        self.profiles.profiles.push(Profile {
            id: id.to_owned(),
            name,
            version,
            asset,
            args,
        });

        if self.profiles.selected.is_empty() {
            self.profiles.selected = id;
        }

        self.save();
    }

    pub fn edit_profile(&mut self, id: String, name: String, version: String) {
        for p in self.profiles.profiles.iter_mut() {
            if p.id == id {
                p.name = name.to_owned();
                p.version = version.to_owned();
            }
        }

        self.save();
    }

    pub fn delete_profile(&mut self, id: String) {
        let index = self
            .profiles
            .profiles
            .iter()
            .position(|p| *p.id == id)
            .unwrap();
        self.profiles.profiles.remove(index);
        self.save();
    }
}

impl Default for Settings {
    fn default() -> Settings {
        Settings {
            auth: Auth {
                username: "".to_string(),
                access_token: "".to_string(),
                client_token: "".to_string(),
                online: false,
            },
            profiles: Profiles {
                selected: "".to_string(),
                profiles: Vec::new(),
            },
            app_dirs: None,
        }
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

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Profile {
    pub id: String,
    pub name: String,
    pub version: String,
    pub asset: String,
    pub args: String,
}
