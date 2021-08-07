use reqwest;
use std::fs::{create_dir_all, File};
use std::path::Path;
use std::process::{Command, Stdio};
use crate::structs::*;
use crate::constants::*;
use crate::utils::files;
use std::io::{BufReader, BufRead};
use std::io::Read;
use crossbeam_channel::Sender;

pub fn prepare_game(profile_id: &str, sender: Sender<String>) {
    let settings = crate::SETTINGS.lock().unwrap();
    let username = settings.auth.username.to_owned();
    std::mem::drop(settings);

    let profile = crate::universal::get_profile(profile_id);
    if profile.is_none() {
        return;
    }

    let profile = profile.unwrap();

    if *crate::CONNECTION.lock().unwrap() {
       let versions_resp: versions::Versions = reqwest::get(VERSIONS).unwrap().json().unwrap();

       for v in versions_resp.versions {
           if v.id == profile.version {
               sender.send("Verifying files".to_string()).unwrap();
               let to_download = files::verify_files(reqwest::get(v.url.as_str()).unwrap().json().unwrap(), &profile.name);

               sender.send("Downloading files".to_string()).unwrap();
               for (k, v) in &to_download {
                   files::download_file(k.to_string(), v);
               }
           }
       }
   } else {
       //TODO should verify files but not download them
       unimplemented!();
   }

    gen_run_cmd(
        format!("{}/profiles/{}", std::env::var("DOT_MCTUI").unwrap(), profile.name).as_str(),
        "/home/noituri/Development/lwjgl-2.9.3/native/linux/",
        &username,
        &profile.version,
        &profile.asset,
        &profile.args,
        sender.clone()
    );
}

//TODO yeah ikr code duplication
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
                let file_name= artifact.path.to_owned().unwrap();
                let file_name = file_name.split("/").last().unwrap();

                libs.push_str(format!("{}/{}/{}:", path, artifact.path.to_owned().unwrap(), file_name).as_str());
            },
            None => {}
        }

        match &lib.downloads.classifiers {
            Some(classifiers) => {
                #[cfg(target_os = "linux")]
                    match &classifiers.natives_linux {
                    Some(native) => {
                        let file_name= native.path.to_owned().unwrap();
                        let file_name = file_name.split("/").last().unwrap();

                        libs.push_str(format!("{}/{}/{}:", path, native.path.to_owned().unwrap(), file_name).as_str());
                    },
                    None => {}
                }

                #[cfg(target_os = "macos")]
                    match &classifiers.natives_osx {
                    Some(native) => {
                        let file_name= native.path.to_owned().unwrap();
                        let file_name = file_name.split("/").last().unwrap();

                        libs.push_str(format!("{}/{}/{}:", path, native.path.to_owned().unwrap(), file_name).as_str());
                    },
                    None => {}
                }

                #[cfg(target_os = "windows")]

                    match &classifiers.natives_windows {
                    Some(native) => {
                        let file_name= native.path.to_owned().unwrap();
                        let file_name = file_name.split("/").last().unwrap();

                        libs.push_str(format!("{}/{}/{}:", path, native.path.to_owned().unwrap(), file_name).as_str());
                    },
                    None => {}
                }
            },
            None => {}
        }
    }

    libs = libs.trim_end_matches(":").to_string();
    Some(libs)
}

pub fn gen_run_cmd(
    profile: &str,
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

    let cmd_arguments = [
        args.to_string(),
        "-XX:HeapDumpPath=MojangTricksIntelDriversForPerformance_javaw.exe_minecraft.exe.heapdump"
            .to_string(),
        format!("-Djava.library.path={}", natives),
        "-Dminecraft.launcher.brand=java-minecraft-launcher".to_string(),
        "-Dminecraft.launcher.version=1.6.89-j".to_string(),
        "-cp".to_string(),
        format!("{}:{}/client.jar", libs, profile),
        "net.minecraft.client.main.Main".to_string(),
        format!("--username={}", username),
        format!("--version='{} MCTui'", version),
        "--accessToken 0".to_string(),
        "--userProperties={{}}".to_string(),
        format!("--gameDir={}", game_dir),
        format!("--assetsDir={}", assets),
        format!("--assetIndex={}", asset_index),
        "--width=1280".to_string(),
        "--height=720".to_string(),
    ];

    let mut cmd = Command::new("java")
        .args(cmd_arguments)
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
