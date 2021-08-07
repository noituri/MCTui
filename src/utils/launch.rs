use crate::constants::*;
use crate::structs::*;
use crate::utils::files;
use futures::{FutureExt, future::{self, join_all}};
use reqwest;
use std::fs::{create_dir_all, File};
use std::io::Read;
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::process::{Command, Stdio};
use crossbeam_channel::Sender;

pub async fn prepare_game(profile_id: &str, sender: Sender<String>) {
    let username = {
        let settings = crate::SETTINGS.lock().unwrap();
        settings.auth.username.to_owned()
    };

    let profile = crate::universal::get_profile(profile_id);
    if profile.is_none() {
        return;
    }

    let profile = profile.unwrap();

    if *crate::CONNECTION.lock().unwrap() {
        let versions_resp: versions::Versions =
            reqwest::get(VERSIONS).await.unwrap().json().await.unwrap();

        for v in versions_resp.versions {
            if v.id == profile.version {
                sender.send("Verifying files".to_string()).unwrap();
                let to_download = files::verify_files(
                    reqwest::get(v.url.as_str())
                        .await
                        .unwrap()
                        .json()
                        .await
                        .unwrap(),
                    &profile.name,
                )
                .await;

                sender.send("Downloading files".to_string()).unwrap();
                let mut download_futures = Vec::new();
                for (k, v) in to_download.iter() {
                    download_futures.push(files::download_file(k.to_string(), v).shared());
                }

                let download_chunks = download_futures.chunks(50);
                let chunks_len = download_chunks.len();
                for (i, chunk) in download_chunks.enumerate() {
                    join_all(chunk.iter().map(|f| f.clone())).await;

                    sender.send(format!("Downloaded {}/{}", i+1, chunks_len-1)).unwrap();
                }
                sender.send("All files downloaded".to_string()).unwrap();
            }
        }
    } else {
        //TODO should verify files but not download them
        unimplemented!();
    }

    gen_run_cmd(
        format!(
            "{}/profiles/{}",
            std::env::var("DOT_MCTUI").unwrap(),
            profile.name
        )
        .as_str(),
        "java",
        "/home/noituri/Development/lwjgl-2.9.3/native/linux/",
        &username,
        &profile.version,
        &profile.asset,
        &profile.args,
        sender.clone(),
    ).await;
}

pub fn gen_libs_path(path: &str, profile: &str) -> Option<String> {
    let libs_path = Path::new(path);

    if !libs_path.exists() || libs_path.is_file() {
        return None;
    }

    let mut file = File::open(format!("{}/version.json", profile)).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    let version: libraries::Libraries = serde_json::from_str(&contents).unwrap();

    let mut libs = String::new();

    for lib in version.libraries.iter() {
        match &lib.downloads.artifact {
            Some(artifact) => {
                let artifact_path = artifact.path.to_owned().unwrap();
                let file_name = artifact_path.split("/").last().unwrap();

                libs.push_str(format!("{}/{}/{}:", path, artifact_path, file_name).as_str());
            }
            None => {}
        }

        if let Some(natives) = lib.downloads.get_natives() {
            let natives_path = natives.path.to_owned().unwrap();
            let file_name = natives_path.split("/").last().unwrap();

            libs.push_str(format!("{}/{}/{}:", path, natives_path, file_name).as_str());
        }
    }

    libs = libs.trim_end_matches(":").to_string();
    Some(libs)
}

pub async fn gen_run_cmd(
    profile: &str,
    java: &str,
    natives: &str,
    username: &str,
    version: &str,
    asset_index: &str,
    args: &str,
    sender: Sender<String>,
) {
    sender
        .send("Launching Minecraft Instance...".to_string())
        .unwrap();

    let dot = std::env::var("DOT_MCTUI").unwrap();

    let libs = gen_libs_path(format!("{}/libs", dot.to_owned()).as_str(), profile).unwrap();
    let assets = format!("{}/assets", dot);
    let game_dir = format!("{}/game", profile);

    create_dir_all(game_dir.to_owned()).unwrap();
    // TODO: Split this into separate options
    let final_cmd = format!("{} {} -XX:HeapDumpPath=MojangTricksIntelDriversForPerformance_javaw.exe_minecraft.exe.heapdump -Djava.library.path={} -Dminecraft.launcher.brand=java-minecraft-launcher -Dminecraft.launcher.version=1.6.89-j -cp {}:{}/client.jar net.minecraft.client.main.Main --username {} --version '{} MCTui' --accessToken 0 --userProperties {{}} --gameDir {} --assetsDir {} --assetIndex {} --width 1280 --height 720",java, args, natives, libs, profile, username, version, game_dir, assets, asset_index);
    let mut cmd = Command::new("bash")
        .arg("-c")
        .arg(final_cmd)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("failed to execute process");

    {
        let stdout = cmd.stdout.as_mut().unwrap();
        let stdout_reader = BufReader::new(stdout);
        let stdout_lines = stdout_reader.lines();

        for line in stdout_lines {
            sender.send(format!("{}\n", line.unwrap())).unwrap();
        }
    }
}
