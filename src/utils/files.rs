use reqwest;
use reqwest::StatusCode;
use std::fs::File;
use std::fs::create_dir_all;
use std::io;
use std::path::Path;
use sha1::Sha1;
use sha1::Digest;
use crate::constants::*;
use crate::structs::*;
use std::error::Error;
use std::io::Read;
use std::collections::HashMap;
//use futures::executor::block_on;
use std::sync::Mutex;


pub fn download_file(url: String, path: &str) {
    create_dir_all(path).unwrap();

    let url_parts: Vec<&str> = url.split('/').collect();
    let output = Path::new(path).join(url_parts.last().unwrap());

    match reqwest::get(url.as_str()) {
        Ok(mut resp) => {
            match resp.status() {
                StatusCode::OK => (),
                _ => {
                    println!("Could not download this file: {}", url);
                    return;
                },
            }
            let mut file = match File::create(&output) {
                Ok(f) => f,
                Err(err) => {
                    println!("Error occurred while creating file: {} | Error: {}", output.display(), err);
                    return;
                }
            };
            match io::copy(&mut resp, &mut file) {
                Ok(_) => {},//println!("File {} has been downloaded", output.display()),
                Err(err) => println!("Could not download this file: {} | Error: {}", url, err),
            }
        },

        Err(err) => println!("Could not download this file: {} | Error: {}", url, err),
    };
}

fn verify_file_exists<'a>(file_path: &'a str, hash: &'a str, to_download: &'a mut Mutex<HashMap<String, String>>, url: String) {
    let path = Path::new(file_path);
    let mut file_dir = file_path.to_string();
    file_dir.truncate(file_path.rfind("/").unwrap());
    let mut td =  to_download.lock().unwrap();
    if !path.exists() || path.is_dir() {
        td.insert(url, file_dir);
        return;
    }

    let mut file = File::open(file_path).unwrap();
    let mut bytes = Vec::new();

    File::read_to_end(&mut file, &mut bytes).unwrap();

    let mut sha = Sha1::default();
    sha.input(&bytes);
    if format!("{:x}", sha.result()).as_str() != hash {
        td.insert(url, file_dir);
    }
}

pub fn verify_files(libs_resp: libraries::Libraries, profile: &str) -> HashMap<String, String> {
    let mut to_download: Mutex<HashMap<String, String>> = Mutex::new(HashMap::new());
    let assets_resp: assets::Assets = reqwest::get(libs_resp.asset_index.url.as_str()).unwrap().json().unwrap();
    let a_indx_path = format!("{}/assets/indexes", DOT_MCTUI);

    verify_file_exists(
        format!("{}/{}", a_indx_path, libs_resp.asset_index.id).as_str(),
        format!("{}/{}", a_indx_path, libs_resp.asset_index.id).as_str(),
        &mut to_download,
        libs_resp.asset_index.url
    );

    for (_, asset) in &assets_resp.objects {
        let asset_path = format!("{}/assets/objects/{}", DOT_MCTUI, &asset.hash[0..2]);

        verify_file_exists(
            format!("{}/{}", asset_path, &asset.hash).as_str(),
            &asset.hash,
            &mut to_download,
            format!("{}/{}/{}", RESOURCES, &asset.hash[0..2], &asset.hash)
        );
    }

    let client_path = format!("{}/profiles/{}", DOT_MCTUI, profile);
    let client = libs_resp.downloads.client.unwrap();
    verify_file_exists(
        format!("{}/client.jar", client_path).as_str(),
        client.sha1.as_str(),
        &mut to_download,
        client.url
    );

    for lib in libs_resp.libraries.iter() {
        match &lib.downloads.artifact {
            Some(artifact) => {
                let url_parts: Vec<&str> = artifact.url.split('/').collect();

                let artifact_path = format!("{}/libs", DOT_MCTUI);
                verify_file_exists(
                    format!("{}/{}", artifact_path, url_parts.last().unwrap()).as_str(),
                    artifact.sha1.as_str(),
                    &mut to_download,
                    artifact.url.to_owned()
                );
            },
            None => {}
        }

        match &lib.downloads.classifiers {
            Some(classifiers) => {
                #[cfg(target_os = "linux")]
                    match &classifiers.natives_linux {
                    Some(native) => {
                        let url_parts: Vec<&str> = native.url.split('/').collect();

                        let class_path = format!("{}/libs", DOT_MCTUI);
                        verify_file_exists(
                            format!("{}/{}", class_path, url_parts.last().unwrap()).as_str(),
                            native.sha1.as_str(),
                            &mut to_download,
                            native.url.to_owned()
                        );
                    },
                    None => {}
                }

                #[cfg(target_os = "macos")]
                    match &classifiers.natives_osx {
                    Some(native) => {
                        let url_parts: Vec<&str> = native.url.split('/').collect();

                        let class_path = format!("{}/libs", DOT_MCTUI);
                        verify_file_exists(
                            format!("{}/{}", class_path, url_parts.last().unwrap()).as_str(),
                            native.sha1.as_str(),
                            &mut to_download,
                            native.url.to_owned()
                        );
                    },
                    None => {}
                }

                #[cfg(target_os = "windows")]
                    match &classifiers.natives_windows {
                    Some(native) => {
                        let url_parts: Vec<&str> = native.url.split('/').collect();

                        let class_path = format!("{}/libs", DOT_MCTUI);
                        verify_file_exists(
                            format!("{}/{}", class_path, url_parts.last().unwrap()).as_str(),
                            native.sha1.as_str(),
                            &mut to_download,
                            native.url.to_owned()
                        );
                    },
                    None => {}
                }
            },
            None => {}
        }
    }

    let td = to_download.lock().unwrap();
    td.clone()
}