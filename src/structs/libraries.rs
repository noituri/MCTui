use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Libraries {
    #[serde(rename = "assetIndex")]
    pub asset_index: AssetIndex,
    pub downloads: Downloads,
    pub libraries: Vec<Library>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AssetIndex {
    pub id: String,
    pub url: String,
    pub sha1: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Library {
    pub downloads: Downloads,
    pub name: String,
    pub rules: Option<Vec<Rule>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Downloads {
    pub artifact: Option<File>,
    pub server: Option<File>,
    pub client: Option<File>,
    pub classifiers: Option<Classifier>,
}

impl Downloads {
    pub fn get_natives<'a>(&'a self) -> &'a Option<File> {
        match &self.classifiers {
            Some(classifiers) => {
                if cfg!(target_os = "linux") {
                    &classifiers.natives_linux
                } else if cfg!(target_os = "macos") {
                    &classifiers.natives_osx
                } else if cfg!(target_os = "windows") {
                    &classifiers.natives_windows
                } else {
                    &None
                }
            }
            None => &None,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct File {
    pub path: Option<String>,
    pub url: String,
    pub sha1: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Rule {
    pub action: String,
    pub os: Option<Os>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Os {
    pub name: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Classifier {
    #[serde(rename = "natives-linux")]
    pub natives_linux: Option<File>,
    #[serde(rename = "natives-osx")]
    pub natives_osx: Option<File>,
    #[serde(rename = "natives-windows")]
    pub natives_windows: Option<File>,
    pub sources: Option<File>,
}
