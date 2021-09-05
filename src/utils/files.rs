use crate::constants::*;
use crate::structs::assets::Assets;
use crate::structs::libraries::Libraries;
use reqwest;
use sha1::Digest;
use sha1::Sha1;
use std::fs::create_dir_all;
use std::fs::File;
use std::io;
use std::path::Path;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum DownloadError {
    #[error("the file checksum does not match the one expected")]
    InvalidChecksum,
    #[error("http client error")]
    HttpClient(#[from] reqwest::Error),
    #[error("io error")]
    Io(#[from] io::Error),
}

#[derive(Clone, Debug)]
pub struct Download {
    url: String,
    dest_path: String,
    checksum: Option<String>,
}

/// Download the content of an URL to the disk
pub async fn download_file(download: &Download) -> Result<u64, DownloadError> {
    create_dir_all(&download.dest_path).unwrap();

    let url_parts: Vec<&str> = download.url.split('/').collect();
    let output = Path::new(&download.dest_path).join(url_parts.last().unwrap());

    let content = reqwest::get(download.url.as_str())
        .await?
        .error_for_status()?
        .bytes()
        .await?;

    if let Some(hash) = &download.checksum {
        let mut sha = Sha1::default();
        sha.update(&content);
        // FIXME: is there a better way to do this without format!() ?
        let file_hash = format!("{:x}", sha.finalize());

        // Security: Do not copy the file if the checksum is not valid
        if file_hash.as_str() != hash {
            return Err(DownloadError::InvalidChecksum);
        }
    }

    let file = &mut File::create(&output)?;
    Ok(io::copy(&mut content.as_ref(), file)?)
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
    libs_resp: &Libraries,
    assets: &Assets,
    profile: &str,
) -> Vec<Download> {
    let dot = data_dir.to_string_lossy().to_string();

    create_dir_all(format!("{}/profiles/{}", dot, profile)).unwrap();
    let mut to_download = Vec::new();

    let a_indx_path = format!("{}/assets/indexes", dot);

    // FIXME: the URL does not have the .json, but the dest file had.
    // So, the file is always missing
    verify_file_exists(
        format!("{}/{}", a_indx_path, libs_resp.asset_index.id),
        libs_resp.asset_index.sha1.clone(),
        libs_resp.asset_index.url.clone(),
        &mut to_download,
    );

    for asset in assets.objects.values() {
        let asset_path = format!("{}/assets/objects/{}", dot, &asset.hash[0..2]);

        verify_file_exists(
            format!("{}/{}", asset_path, &asset.hash),
            asset.hash.to_owned(),
            format!("{}/{}/{}", RESOURCES, &asset.hash[0..2], &asset.hash),
            &mut to_download,
        );
    }

    let client_path = format!("{}/profiles/{}", dot, profile);
    let client = libs_resp.downloads.client.clone().unwrap();
    verify_file_exists(
        format!("{}/client.jar", client_path),
        client.sha1.clone(),
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
