use reqwest::Error;

use crate::structs::{
    assets::Assets,
    libraries::Libraries,
    versions::{Version, Versions},
};

pub async fn get_versions() -> Result<Versions, Error> {
    reqwest::get("https://launchermeta.mojang.com/mc/game/version_manifest.json")
        .await?
        .json::<Versions>()
        .await
}

pub async fn get_libs(version: &Version) -> Result<Libraries, Error> {
    reqwest::get(version.url.as_str())
        .await?
        .json::<Libraries>()
        .await
}

pub async fn get_assets(libs: &Libraries) -> Result<Assets, Error> {
    reqwest::get(libs.asset_index.url.as_str())
        .await?
        .json::<Assets>()
        .await
}
