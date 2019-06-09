use reqwest;
use std::collections::HashMap;
use serde_json::{Result, Value};

mod utils;
mod responsetypes;

static VERSIONS: &str = "https://launchermeta.mojang.com/mc/game/version_manifest.json";

fn main() {
    let versions_resp: responsetypes::Versions = reqwest::get(VERSIONS).unwrap().json().unwrap();

    for v in versions_resp.versions {
        if v.id == "1.12.2" {
            let libs_resp: responsetypes::Libraries = reqwest::get(v.url.as_str()).unwrap().json().unwrap();
            utils::download_file(libs_resp.downloads.server.unwrap().url, "./data/1.12.2");
            utils::download_file(libs_resp.downloads.client.unwrap().url, "./data/1.12.2");

            for lib in libs_resp.libraries.iter() {
                match &lib.downloads.artifact {
                    Some(artifact) => utils::download_file(artifact.url.to_owned(), "./data/1.12.2/libs"),
                    None => {}
                }

                match &lib.downloads.classifiers {
                    Some(classifiers) => {
                        match &classifiers.natives_linux {
                            Some(native) => utils::download_file(native.url.to_owned(), "./data/1.12.2/libs"),
                            None => {}
                        }
                    },
                    None => {}
                }
            }
        }
    }
}
