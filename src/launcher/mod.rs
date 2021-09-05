pub mod authentication;

use platform_dirs::AppDirs;
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fs::File;
use std::io::Read;
use uuid::Uuid;

use crate::structs::settings::{Profile, Profiles};

use self::authentication::Authentication;

const FILE_NAME: &str = "mctui.json";

#[derive(Debug, Serialize, Deserialize)]
pub struct LauncherAuth {
    authentication: Option<Authentication>,
}

impl LauncherAuth {
    /// Returns the authentication state
    pub fn get(&self) -> Option<&Authentication> {
        self.authentication.as_ref()
    }

    /// Authenticates the user to Minecraft
    pub fn authenticate(&mut self, id: &str, _password: &str) {
        self.authentication = Some(Authentication {
            username: id.to_string(),
            access_token: "0".to_string(),
            client_token: "".to_string(),
        })
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Launcher {
    pub auth: LauncherAuth,
    pub profiles: Profiles,

    /// FIXME: Temporary solution until Launcher refactoring
    #[serde(skip)]
    pub app_dirs: Option<AppDirs>,
}

impl Launcher {
    pub fn new(app_dirs: AppDirs) -> Result<Self, Box<dyn Error>> {
        let settings_path = app_dirs.data_dir.join(FILE_NAME);

        let mut launcher = match settings_path.exists() {
            true => {
                let mut file = File::open(&settings_path)?;
                let mut contents = String::new();
                file.read_to_string(&mut contents)?;

                serde_json::from_str(&contents)?
            }
            false => Launcher::default(),
        };

        // FIXME: Temporary solution until Laucnher refactoring
        launcher.app_dirs = Some(app_dirs);

        Ok(launcher)
    }

    pub fn save(&self) {
        // FIXME: Option<T> Temporary solution until launcher refactoring
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

impl Default for Launcher {
    fn default() -> Launcher {
        Launcher {
            auth: LauncherAuth {
                authentication: None,
            },
            profiles: Profiles {
                selected: "".to_string(),
                profiles: Vec::new(),
            },
            app_dirs: None,
        }
    }
}
