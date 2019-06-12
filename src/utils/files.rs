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
                Ok(_) => println!("File {} has been downloaded", output.display()),
                Err(err) => println!("Could not download this file: {} | Error: {}", url, err),
            }
        },

        Err(err) => println!("Could not download this file: {} | Error: {}", url, err),
    };
}

pub fn verify_file_exists(file_path: &str, hash: &str) -> Result<(), Box<Error>> {
    let path = Path::new(file_path);

    if !path.exists() || path.is_dir() {
        return Err(Box::new(std::io::Error::new(std::io::ErrorKind::InvalidInput, "File exists or is a dir")));
    }

    let mut file = File::open(file_path)?;
    let mut bytes = Vec::new();

    File::read_to_end(&mut file, &mut bytes)?;

    let mut sha = Sha1::default();
    sha.input(&bytes);
    if format!("{:x}", sha.result()).as_str() != hash {
        return Err(Box::new(std::io::Error::new(std::io::ErrorKind::InvalidInput, "hash")));
    }

    return Ok(())
}

pub fn verify_files(libs_resp: libraries::Libraries, profile: &str) {
    let mut to_download: HashMap<String, String> = HashMap::new();
    let assets_resp: assets::Assets = reqwest::get(libs_resp.asset_index.url.as_str()).unwrap().json().unwrap();

    let a_indx_path = format!("{}/profiles/{}/assets/indexes", DOT_MCTUI, profile);
    if verify_file_exists(format!("{}/{}", a_indx_path, libs_resp.asset_index.id).as_str(), libs_resp.asset_index.sha1.as_str()).is_err() {
        to_download.insert(libs_resp.asset_index.url, a_indx_path);
    }

    for (_, asset) in &assets_resp.objects {
        let asset_path = format!("{}/profiles/{}/assets/objects/{}", DOT_MCTUI, profile, &asset.hash[0..2]);

        if verify_file_exists(format!("{}/{}", asset_path, &asset.hash).as_str(), &asset.hash).is_err() {
            to_download.insert(format!("{}/{}/{}", RESOURCES, &asset.hash[0..2], &asset.hash), asset_path);
        }
    }

    let client_path = format!("{}/profiles/{}", DOT_MCTUI, profile);
    let client = libs_resp.downloads.client.unwrap();
    if verify_file_exists(format!("{}/client.jar", client_path).as_str(), client.sha1.as_str()).is_err() {
        to_download.insert(client.url, client_path);
    }

    for lib in libs_resp.libraries.iter() {
        match &lib.downloads.artifact {
            Some(artifact) => {
                let url_parts: Vec<&str> = artifact.url.split('/').collect();

                let artifact_path = format!("{}/profiles/{}/libs", DOT_MCTUI, profile);
                if verify_file_exists(format!("{}/{}", artifact_path, url_parts.last().unwrap()).as_str(), artifact.sha1.as_str()).is_err() {
                    to_download.insert(artifact.url.to_owned(), artifact_path);
                }
            },
            None => {}
        }

        match &lib.downloads.classifiers {
            Some(classifiers) => {
                #[cfg(target_os = "linux")]
                    match &classifiers.natives_linux {
                    Some(native) => {
                        let url_parts: Vec<&str> = native.url.split('/').collect();

                        let class_path = format!("{}/profiles/{}/libs", DOT_MCTUI, profile);
                        if verify_file_exists(format!("{}/{}", class_path, url_parts.last().unwrap()).as_str(), native.sha1.as_str()).is_err() {
                            to_download.insert(native.url.to_owned(), class_path);
                        }
                    },
                    None => {}
                }

                #[cfg(target_os = "macos")]
                    match &classifiers.natives_osx {
                    Some(native) => {
                        let url_parts: Vec<&str> = native.url.split('/').collect();

                        let class_path = format!("{}/profiles/{}/libs", DOT_MCTUI, profile);
                        if verify_file_exists(format!("{}/{}", class_path, url_parts.last().unwrap()).as_str(), native.sha1.as_str()).is_err() {
                            to_download.insert(native.url.to_owned(), class_path);
                        }
                    },
                    None => {}
                }

                #[cfg(target_os = "windows")]
                    match &classifiers.natives_windows {
                    Some(native) => {
                        let url_parts: Vec<&str> = native.url.split('/').collect();

                        let class_path = format!("{}/profiles/{}/libs", DOT_MCTUI, profile);
                        if verify_file_exists(format!("{}/{}", class_path, url_parts.last().unwrap()).as_str(), native.sha1.as_str()).is_err() {
                            to_download.insert(native.url.to_owned(), class_path);
                        }
                    },
                    None => {}
                }
            },
            None => {}
        }
    }

    for (k, v) in &to_download {
        download_file(k.to_owned(), v);
    }
}