use crate::constants::*;
use crate::structs::assets::Assets;
use crate::structs::libraries::Libraries;
use crate::structs::versions::Version;
use reqwest;
use sha1::Digest;
use sha1::Sha1;
use std::fs::create_dir_all;
use std::fs::File;
use std::io;
use std::path::Path;
use std::path::PathBuf;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum DownloadError {
    #[error("the file checksum does not match the one expected")]
    InvalidChecksum,
    #[error("File system error: {0}")]
    Fs(String),
    #[error("Reqwest client error")]
    Reqwest(#[from] reqwest::Error),
    #[error("IO error")]
    Io(#[from] io::Error),
}

#[derive(Clone, Debug)]
pub struct Download {
    url: String,
    dest_path: PathBuf,
    checksum: Option<String>,
}

/// Download the content of an URL to the disk
pub async fn download_file(download: &Download) -> Result<u64, DownloadError> {
    create_dir_all(&download.dest_path.parent().ok_or_else(|| {
        DownloadError::Fs("Unable to get the parent directory of the destination".to_string())
    })?)?;

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

    let file = &mut File::create(&download.dest_path)?;
    Ok(io::copy(&mut content.as_ref(), file)?)
}

fn verify_file_exists(
    dest_path: PathBuf,
    hash: String,
    url: String,
    to_download: &mut Vec<Download>,
) {
    if !dest_path.is_file() {
        to_download.push(Download {
            url,
            dest_path,
            checksum: Some(hash),
        });
    }
}

/// @ref: https://minecraft.fandom.com/wiki/.minecraft
pub async fn verify_files(
    data_dir: &Path,
    version: &Version,
    libs_resp: &Libraries,
    assets: &Assets,
) -> Vec<Download> {
    let mut to_download = Vec::new();

    // Assets
    // --------------------------------------------------------------------------------
    let assets_path = data_dir.join("assets");

    verify_file_exists(
        assets_path
            .join("indexes")
            .join(libs_resp.asset_index.id.clone() + ".json"),
        libs_resp.asset_index.sha1.clone(),
        libs_resp.asset_index.url.clone(),
        &mut to_download,
    );

    let assets_obj_path = assets_path.join("objects");
    for asset in assets.objects.values() {
        verify_file_exists(
            assets_obj_path.join(&asset.hash[0..2]).join(&asset.hash),
            asset.hash.to_owned(),
            format!("{}/{}/{}", RESOURCES, &asset.hash[0..2], &asset.hash),
            &mut to_download,
        );
    }

    // Client
    // --------------------------------------------------------------------------------
    let version_path = data_dir.join("versions").join(version.id.clone());
    create_dir_all(&version_path).unwrap();

    let client = libs_resp.downloads.client.clone().unwrap();

    verify_file_exists(
        version_path.join("client.jar"),
        client.sha1.clone(),
        client.url,
        &mut to_download,
    );

    // Libs
    // --------------------------------------------------------------------------------
    let libs_path = data_dir.join("libraries");

    for lib in libs_resp.libraries.iter() {
        match &lib.downloads.artifact {
            Some(artifact) => {
                verify_file_exists(
                    libs_path.join(artifact.path.clone().unwrap()),
                    artifact.sha1.to_owned(),
                    artifact.url.to_owned(),
                    &mut to_download,
                );
            }
            None => {}
        }

        if let Some(natives) = lib.downloads.get_natives() {
            verify_file_exists(
                libs_path.join(natives.path.clone().unwrap()),
                natives.sha1.to_owned(),
                natives.url.to_owned(),
                &mut to_download,
            );
        }
    }

    to_download
}
