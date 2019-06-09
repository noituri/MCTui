use reqwest;
use std::collections::HashMap;
use serde_json::{Result, Value};

mod utils;
mod responsetypes;

static VERSIONS: &str = "https://launchermeta.mojang.com/mc/game/version_manifest.json";
static RESOURCES: &str = "http://resources.download.minecraft.net";
static PROFILE: &str = "Default14";
static USERNAME: &str = "Vedmak";
static DOT_MCTUI: &str = "/home/noituri/.mctui";

fn main() {
    let versions_resp: responsetypes::Versions = reqwest::get(VERSIONS).unwrap().json().unwrap();

    for v in versions_resp.versions {
        if v.id == "1.14.2" {
            let libs_resp: responsetypes::Libraries = reqwest::get(v.url.as_str()).unwrap().json().unwrap();
            let assets_resp: responsetypes::Assets = reqwest::get(libs_resp.asset_index.url.as_str()).unwrap().json().unwrap();

            utils::download_file(libs_resp.asset_index.url, format!("{}/data/profiles/{}/assets/indexes", DOT_MCTUI, PROFILE).as_str());

            for (_, asset) in &assets_resp.objects {
//             break;
                utils::download_file(format!("{}/{}/{}", RESOURCES, &asset.hash[0..2], &asset.hash), format!("{}/data/profiles/{}/assets/objects/{}", DOT_MCTUI, PROFILE, &asset.hash[0..2]).as_str());
            }

            utils::download_file(libs_resp.downloads.client.unwrap().url, format!("{}/data/profiles/{}", DOT_MCTUI, PROFILE).as_str());

            for lib in libs_resp.libraries.iter() {
//             break;
                match &lib.downloads.artifact {
                    Some(artifact) => utils::download_file(artifact.url.to_owned(), format!("{}/data/profiles/{}/libs", DOT_MCTUI, PROFILE).as_str()),
                    None => {}
                }

                match &lib.downloads.classifiers {
                    Some(classifiers) => {
                        match &classifiers.natives_linux {
                            Some(native) => utils::download_file(native.url.to_owned(), format!("{}/data/profiles/{}/libs", DOT_MCTUI, PROFILE).as_str()),
                            None => {}
                        }
                    },
                    None => {}
                }
            }

            utils::gen_run_cmd(
                format!("{}/data/profiles/{}", DOT_MCTUI, PROFILE).as_str(),
                "/usr/bin/java",
                "/usr/share/lwjgl2/native/linux",
                USERNAME,
                &v.id
            );
        }
    }
}
