use crate::constants::*;
use crate::structs::*;
use futures::future::join_all;
use reqwest;
use reqwest::StatusCode;
use sha1::Digest;
use sha1::Sha1;
use std::fs::create_dir_all;
use std::fs::File;
use std::io;
use std::io::Read;
use std::path::Path;
use std::sync::Mutex;
use std::{collections::HashMap, sync::Arc};

pub async fn download_file(url: String, path: &str) {
    create_dir_all(path).unwrap();

    let url_parts: Vec<&str> = url.split('/').collect();
    let output = Path::new(path).join(url_parts.last().unwrap());

    match reqwest::get(url.as_str()).await {
        Ok(resp) => {
            match resp.status() {
                StatusCode::OK => (),
                _ => {
                    println!("Could not download this file: {}", url);
                    return;
                }
            }
            let mut file = match File::create(&output) {
                Ok(f) => f,
                Err(err) => {
                    println!(
                        "Error occurred while creating file: {} | Error: {}",
                        output.display(),
                        err
                    );
                    return;
                }
            };

            let bytes = resp.bytes().await.unwrap();
            match io::copy(&mut bytes.as_ref(), &mut file) {
                Ok(_) => {} //println!("File {} has been downloaded", output.display()),
                Err(err) => println!("Could not download this file: {} | Error: {}", url, err),
            }
        }

        Err(err) => println!("Could not download this file: {} | Error: {}", url, err),
    };
}

async fn verify_file_exists(
    file_path: String,
    hash: String,
    to_download: Arc<Mutex<HashMap<String, String>>>,
    url: String,
) {
    let path = Path::new(&file_path);
    let mut file_dir = file_path.to_string();
    file_dir.truncate(file_path.rfind('/').unwrap());
    {
        let mut td = to_download.lock().unwrap();
        if !path.exists() || path.is_dir() {
            td.insert(url, file_dir);
            return;
        }
    }

    let mut file = File::open(file_path).unwrap();
    let mut bytes = Vec::new();

    File::read_to_end(&mut file, &mut bytes).unwrap();

    let mut sha = Sha1::default();
    sha.update(&bytes);
    if format!("{:x}", sha.finalize()).as_str() != hash {
        let mut td = to_download.lock().unwrap();
        td.insert(url, file_dir);
    }
}

pub async fn verify_files(
    libs_resp: libraries::Libraries,
    profile: &str,
) -> HashMap<String, String> {
    let dot = std::env::var("DOT_MCTUI").unwrap();

    create_dir_all(format!("{}/profiles/{}", dot.to_owned(), profile)).unwrap();
    serde_json::to_writer_pretty(
        &File::create(format!(
            "{}/profiles/{}/version.json",
            dot.to_owned(),
            profile
        ))
        .unwrap(),
        &libs_resp,
    )
    .unwrap();

    let mut verify_futures = Vec::new();
    let to_download: Arc<Mutex<HashMap<String, String>>> = Arc::new(Mutex::new(HashMap::new()));

    let assets_resp: assets::Assets = reqwest::get(libs_resp.asset_index.url.as_str())
        .await
        .unwrap()
        .json()
        .await
        .unwrap();
    let a_indx_path = format!("{}/assets/indexes", dot.to_owned());

    verify_futures.push(verify_file_exists(
        format!("{}/{}", a_indx_path, libs_resp.asset_index.id),
        format!("{}/{}", a_indx_path, libs_resp.asset_index.id),
        to_download.clone(),
        libs_resp.asset_index.url,
    ));

    for asset in assets_resp.objects.values() {
        let asset_path = format!("{}/assets/objects/{}", dot.to_owned(), &asset.hash[0..2]);

        verify_futures.push(verify_file_exists(
            format!("{}/{}", asset_path, &asset.hash),
            asset.hash.to_owned(),
            to_download.clone(),
            format!("{}/{}/{}", RESOURCES, &asset.hash[0..2], &asset.hash),
        ));
    }

    let client_path = format!("{}/profiles/{}", dot.to_owned(), profile);
    let client = libs_resp.downloads.client.unwrap();
    verify_futures.push(verify_file_exists(
        format!("{}/client.jar", client_path),
        client.sha1,
        to_download.clone(),
        client.url,
    ));

    for lib in libs_resp.libraries.iter() {
        match &lib.downloads.artifact {
            Some(artifact) => {
                let url_parts: Vec<&str> = artifact.url.split('/').collect();

                let artifact_path = format!(
                    "{}/libs/{}",
                    dot.to_owned(),
                    artifact.path.to_owned().unwrap()
                );
                verify_futures.push(verify_file_exists(
                    format!("{}/{}", artifact_path, url_parts.last().unwrap()),
                    artifact.sha1.to_owned(),
                    to_download.clone(),
                    artifact.url.to_owned(),
                ));
            }
            None => {}
        }

        if let Some(natives) = lib.downloads.get_natives() {
            let url_parts: Vec<&str> = natives.url.split('/').collect();

            let class_path = format!(
                "{}/libs/{}",
                dot.to_owned(),
                natives.path.to_owned().unwrap()
            );
            verify_futures.push(verify_file_exists(
                format!("{}/{}", class_path, url_parts.last().unwrap()),
                natives.sha1.to_owned(),
                to_download.clone(),
                natives.url.to_owned(),
            ));
        }
    }

    join_all(verify_futures).await;

    let td = to_download.lock().unwrap();
    td.clone()
}
