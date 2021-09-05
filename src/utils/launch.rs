use crate::constants::*;
use crate::launcher::authentication::Authentication;
use crate::launcher::profile::Profile;
use crate::structs::libraries::Libraries;
use crate::structs::*;
use crate::utils::files;
use crossbeam_channel::Sender;
use futures::{stream, StreamExt};
use reqwest;
use std::fs::{create_dir_all, File};
use std::io::Read;
use std::io::{BufRead, BufReader};
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

const LIB_SEPARATOR: &str = if cfg!(target_os = "windows") {
    ";"
} else {
    ":"
};

pub async fn prepare_game(
    data_dir: &Path,
    profile: &Profile,
    authentication: &Authentication,
    sender: Sender<String>,
) {
    let versions_resp: versions::Versions =
        reqwest::get(VERSIONS).await.unwrap().json().await.unwrap();

    for v in versions_resp.versions {
        if v.id == profile.version {
            sender.send("Verifying files".to_string()).unwrap();
            let to_download = files::verify_files(
                data_dir,
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

            let total_to_dl = &to_download.len();
            let mut total_dl = 0;
            let stream_sender = &sender.clone();

            stream::iter(to_download)
                .map(|(k, v)| async move {
                    files::download_file(k.to_string(), &v).await;
                })
                .buffer_unordered(5)
                .map(|_| {
                    total_dl += 1;
                    stream_sender
                        .send(format!("Downloaded: {}%", total_dl * 100 / total_to_dl))
                        .unwrap();
                })
                .collect::<Vec<_>>()
                .await;

            sender.send("All files downloaded".to_string()).unwrap();
        }
    }

    gen_run_cmd(
        data_dir,
        &format!("{}/profiles/{}", data_dir.to_string_lossy(), profile.name),
        authentication,
        &profile.version,
        &profile.asset,
        &profile.args,
        sender.clone(),
    )
    .await;
}

/// Loads the librairies from the given profile
fn load_game_libs(profile: &str) -> Libraries {
    let mut file = File::open(format!("{}/version.json", profile)).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    serde_json::from_str(&contents).unwrap()
}

// Lists all the librairies needed to launch the game
fn list_libs_path(path: &str, profile: &str) -> Option<Vec<PathBuf>> {
    let libs_path = Path::new(path);

    if !libs_path.exists() || libs_path.is_file() {
        return None;
    }

    let version = load_game_libs(profile);
    let mut libs = Vec::new();

    for lib in version.libraries.iter() {
        match &lib.downloads.artifact {
            Some(artifact) => {
                let artifact_path = artifact.path.to_owned().unwrap();
                let file_name = &artifact_path.split('/').last().unwrap();

                libs.push(libs_path.join(&artifact_path).join(file_name));
            }
            None => {}
        }

        if let Some(natives) = lib.downloads.get_natives() {
            let natives_path = natives.path.to_owned().unwrap();
            let file_name = &natives_path.split('/').last().unwrap();

            libs.push(libs_path.join(&natives_path).join(file_name));
        }
    }

    Some(libs)
}

pub async fn gen_run_cmd(
    data_dir: &Path,
    profile: &str,
    // natives: &str,
    authntication: &Authentication,
    version: &str,
    asset_index: &str,
    args: &str,
    sender: Sender<String>,
) {
    sender
        .send("Launching Minecraft Instance...".to_string())
        .unwrap();

    let dot = data_dir.to_string_lossy().to_string();
    let base_dir = Path::new(&dot);
    let profile_dir = Path::new(&profile);

    let mut libs = list_libs_path(format!("{}/libs", dot).as_str(), profile).unwrap();
    libs.push(profile_dir.join("client.jar"));

    let assets = base_dir.join("assets");
    let game_dir = profile_dir.join("game");

    create_dir_all(game_dir.to_owned()).unwrap();

    let mut args = args.split(' ').map(|a| a.to_owned()).collect::<Vec<_>>();

    let libs_args = libs
        .iter()
        .map(|x| x.to_string_lossy().to_string())
        .collect::<Vec<String>>()
        .join(LIB_SEPARATOR);

    let additional_arguments = [
        "-XX:HeapDumpPath=MojangTricksIntelDriversForPerformance_javaw.exe_minecraft.exe.heapdump"
            .to_string(),
        // format!("-Djava.library.path={}", natives),
        "-Dminecraft.launcher.brand=java-minecraft-launcher".to_string(),
        "-Dminecraft.launcher.version=1.6.89-j".to_string(),
        "-cp".to_string(),
        libs_args,
        "net.minecraft.client.main.Main".to_string(),
        format!("--username={}", authntication.username),
        format!("--accessToken={}", authntication.access_token),
        format!("--version='{} MCTui'", version),
        // "--userProperties={{}}".to_string(),
        format!("--gameDir={}", game_dir.to_string_lossy()),
        format!("--assetsDir={}", assets.to_string_lossy()),
        format!("--assetIndex={}", asset_index),
        "--width=1280".to_string(),
        "--height=720".to_string(),
    ];

    args.extend(additional_arguments);

    let mut cmd = Command::new("java")
        .args(args)
        .current_dir(data_dir)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("failed to execute process");

    {
        let stdout = cmd.stdout.as_mut().unwrap();
        let stdout_reader = BufReader::new(stdout);
        let stdout_lines = stdout_reader.lines();

        let stderr = cmd.stderr.as_mut().unwrap();
        let stderr_reader = BufReader::new(stderr);
        let stderr_lines = stderr_reader.lines();

        for line in stdout_lines {
            sender.send(format!("{}\n", line.unwrap())).unwrap();
        }

        for line in stderr_lines {
            sender.send(format!("{}\n", line.unwrap())).unwrap();
        }
    }
}
