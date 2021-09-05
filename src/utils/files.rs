use crate::constants::*;
use crate::structs::*;
use reqwest;
use reqwest::StatusCode;
use sha1::Digest;
use sha1::Sha1;
use std::fs::create_dir_all;
use std::fs::File;
use std::io;
use std::path::Path;

pub struct Download {
    url: String,
    dest_path: String,
    checksum: Option<String>,
}

pub async fn download_file(download: &Download) {
    create_dir_all(&download.dest_path).unwrap();

    let url_parts: Vec<&str> = download.url.split('/').collect();
    let output = Path::new(&download.dest_path).join(url_parts.last().unwrap());

    match reqwest::get(download.url.as_str()).await {
        Ok(resp) => {
            match resp.status() {
                StatusCode::OK => (),
                _ => {
                    println!("Could not download this file: {}", download.url);
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

            if let Some(hash) = &download.checksum {
                let mut sha = Sha1::default();
                sha.update(&bytes);
                // FIXME: is there a better way to do this without format!() ?
                let file_hash = format!("{:x}", sha.finalize());

                // Security: Do not copy the file if the checksum is not valid
                if file_hash.as_str() == hash {
                    match io::copy(&mut bytes.as_ref(), &mut file) {
                        Ok(_) => {} //println!("File {} has been downloaded", output.display()),
                        Err(err) => {
                            println!(
                                "Could not download this file: {} | Error: {}",
                                download.url, err
                            )
                        }
                    }
                }
            }
        }

        Err(err) => println!(
            "Could not download this file: {} | Error: {}",
            download.url, err
        ),
    };
}

fn verify_file_exists(
    file_path: String,
    hash: String,
    url: String,
    to_download: &mut Vec<Download>,
) {
    let path = Path::new(&file_path);
    let mut file_dir = file_path.to_string();
    file_dir.truncate(file_path.rfind('/').unwrap());

    if !path.is_file() {
        to_download.push(Download {
            url,
            dest_path: file_dir,
            checksum: Some(hash),
        });
    }
}

pub async fn verify_files(
    data_dir: &Path,
    libs_resp: libraries::Libraries,
    profile: &str,
) -> Vec<Download> {
    let dot = data_dir.to_string_lossy().to_string();

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

    let mut to_download = Vec::new();

    let assets_resp: assets::Assets = reqwest::get(libs_resp.asset_index.url.as_str())
        .await
        .unwrap()
        .json()
        .await
        .unwrap();
    let a_indx_path = format!("{}/assets/indexes", dot.to_owned());

    verify_file_exists(
        format!("{}/{}", a_indx_path, libs_resp.asset_index.id),
        format!("{}/{}", a_indx_path, libs_resp.asset_index.id),
        libs_resp.asset_index.url,
        &mut to_download,
    );

    for asset in assets_resp.objects.values() {
        let asset_path = format!("{}/assets/objects/{}", dot.to_owned(), &asset.hash[0..2]);

        verify_file_exists(
            format!("{}/{}", asset_path, &asset.hash),
            asset.hash.to_owned(),
            format!("{}/{}/{}", RESOURCES, &asset.hash[0..2], &asset.hash),
            &mut to_download,
        );
    }

    let client_path = format!("{}/profiles/{}", dot.to_owned(), profile);
    let client = libs_resp.downloads.client.unwrap();
    verify_file_exists(
        format!("{}/client.jar", client_path),
        client.sha1,
        client.url,
        &mut to_download,
    );

    for lib in libs_resp.libraries.iter() {
        match &lib.downloads.artifact {
            Some(artifact) => {
                let url_parts: Vec<&str> = artifact.url.split('/').collect();

                let artifact_path = format!(
                    "{}/libs/{}",
                    dot.to_owned(),
                    artifact.path.to_owned().unwrap()
                );
                verify_file_exists(
                    format!("{}/{}", artifact_path, url_parts.last().unwrap()),
                    artifact.sha1.to_owned(),
                    artifact.url.to_owned(),
                    &mut to_download,
                );
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
            verify_file_exists(
                format!("{}/{}", class_path, url_parts.last().unwrap()),
                natives.sha1.to_owned(),
                natives.url.to_owned(),
                &mut to_download,
            );
        }
    }

    to_download
}
