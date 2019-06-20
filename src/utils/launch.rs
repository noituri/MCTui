use reqwest;
use std::fs;
use std::fs::create_dir_all;
use std::path::Path;
use std::process::{Command, Stdio};
use crate::structs::*;
use crate::constants::*;
use crate::utils::files;
use std::io::{BufReader, BufRead};
use crossbeam_channel::Sender;
//use futures::executor::block_on;

pub fn prepare_game(profile_id: &str, sender: Sender<String>) {

    let settings = crate::SETTINGS.lock().unwrap();
    let profile = settings.profiles.get_profile(profile_id);
    if profile.is_none() {
        return;
    }

    let profile = profile.unwrap();


//   if *crate::CONNECTION.lock().unwrap() {
//       let versions_resp: versions::Versions = reqwest::get(VERSIONS).unwrap().json().unwrap();
//
//       for v in versions_resp.versions {
//           if v.id == profile.version {
//               let to_download = files::verify_files(reqwest::get(v.url.as_str()).unwrap().json().unwrap(), &profile.name);
//
//               for (k, v) in &to_download {
//                   files::download_file(k.to_string(), v)
//               }
//           }
//       }
//   }

    gen_run_cmd(
        format!("{}/profiles/{}", DOT_MCTUI, profile.name).as_str(),
        "java",
        "/home/noituri/Development/lwjgl-2.9.3/native/linux",
        &settings.auth.username,
        &profile.version,
        &profile.asset,
        sender
    );
}

pub fn gen_libs_path(path: &str) -> Option<String> {
    let libs_path = Path::new(path);

    if !libs_path.exists() || libs_path.is_file() {
        return None;
    }

    let mut libs = String::new();
    for entry in fs::read_dir(libs_path).unwrap() {
        if let Ok(entry) = entry {
            if entry.path().is_file() {
                libs.push_str(format!("{}/{}:", path, entry.file_name().to_str().unwrap()).as_str());
            }
        }
    }

    libs = libs.trim_end_matches(":").to_string();
    Some(libs)
}

pub fn gen_run_cmd(profile: &str, java: &str, natives: &str, username: &str, version: &str, asset_index: &str, sender: Sender<String>) {
  //  println!("Launching Minecraft Instance...");
    let libs = gen_libs_path(format!("{}/libs", DOT_MCTUI).as_str()).unwrap();
    let assets = format!("{}/assets", DOT_MCTUI);
    let game_dir = format!("{}/game", profile);

    create_dir_all(game_dir.to_owned()).unwrap();
    // TODO: Split this into separate options
    let final_cmd = format!("{} -Xmx1G -XX:+UseConcMarkSweepGC -XX:+CMSIncrementalMode -XX:-UseAdaptiveSizePolicy -Xmn128M -XX:HeapDumpPath=MojangTricksIntelDriversForPerformance_javaw.exe_minecraft.exe.heapdump -Djava.library.path={} -Dminecraft.launcher.brand=java-minecraft-launcher -Dminecraft.launcher.version=1.6.89-j -cp {}:{}/client.jar net.minecraft.client.main.Main --username {} --version '{} MCTui' --accessToken 0 --userProperties {{}} --gameDir {} --assetsDir {} --assetIndex {} --width 1280 --height 720",java, natives, libs, profile, username, version, game_dir, assets, asset_index);

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
