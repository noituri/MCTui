use serde::{Deserialize, Serialize};

// Versions
#[derive(Serialize, Deserialize)]
pub struct Versions {
    pub latest: Latest,
    pub versions: Vec<Version>
}

#[derive(Serialize, Deserialize)]
pub struct Latest {
    pub release: String,
    pub snapshot: String
}

#[derive(Serialize, Deserialize)]
pub struct Version {
    pub id: String,
    #[serde(rename = "type")]
    pub v_type: String,
    pub url: String,
    pub time : String,
    pub releaseTime: String
}

// Libraries
#[derive(Serialize, Deserialize, Debug)]
pub struct Libraries {
    pub assets: String,
    pub downloads: Downloads,
    pub libraries: Vec<Library>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Library {
    pub downloads: Downloads,
    pub name: String,
    pub rules: Option<Vec<Rule>>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Downloads {
    pub artifact: Option<File>,
    pub server: Option<File>,
    pub client: Option<File>,
    pub classifiers: Option<Classifier>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct File {
    pub path: Option<String>,
    pub url: String
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Rule {
    pub action: String,
    pub os: Option<Os>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Os {
    pub name: String
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Classifier {
    #[serde(rename = "natives-linux")]
    pub natives_linux: Option<File>,
    #[serde(rename = "natives-osx")]
    pub natives_osx: Option<File>,
    #[serde(rename = "natives-windows")]
    pub natives_windows: Option<File>,
    pub sources: Option<File>
}
