use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Versions {
    pub latest: Latest,
    pub versions: Vec<Version>,
}

#[derive(Serialize, Deserialize)]
pub struct Latest {
    pub release: String,
    pub snapshot: String,
}

#[derive(Serialize, Deserialize)]
pub struct Version {
    pub id: String,
    #[serde(rename = "type")]
    pub v_type: String,
    pub url: String,
    pub time: DateTime<Utc>,
    #[serde(rename = "releaseTime")]
    pub release_date: DateTime<Utc>,
}
