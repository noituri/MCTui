use reqwest;
use reqwest::StatusCode;
use std::fs;
use std::fs::File;
use std::fs::create_dir_all;
use std::io;
use std::path::Path;
use std::process::Command;
use crate::structs::*;
use crate::constants::*;
use crate::utils::files;

static PROFILE: &str = "Default14";
static USERNAME: &str = "Vedmak";

pub fn prepare_game() {
    let versions_resp: versions::Versions = reqwest::get(VERSIONS).unwrap().json().unwrap();

    for v in versions_resp.versions {
        if v.id == "1.14.2" {
            files::download_basic_game(reqwest::get(v.url.as_str()).unwrap().json().unwrap(), PROFILE);

            gen_run_cmd(
                format!("{}/profiles/{}", DOT_MCTUI, PROFILE).as_str(),
                "/usr/bin/java",
                "/usr/share/lwjgl2/native/linux",
                USERNAME,
                &v.id
            );
        }
    }
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
    return Some(libs)
}

pub fn gen_run_cmd(profile: &str, java: &str, natives: &str, username: &str, version: &str) {
    println!("Launching Minecraft Instance...");
    let libs = gen_libs_path(format!("{}/libs", profile).as_str()).unwrap();
    let assets = format!("{}/assets", profile);
    let game_dir = format!("{}/game", profile);

    create_dir_all(game_dir.to_owned());
    // TODO: Split this into separate options
    let cmd = format!("{} -Xmx1G -XX:+UseConcMarkSweepGC -XX:+CMSIncrementalMode -XX:-UseAdaptiveSizePolicy -Xmn128M -XX:HeapDumpPath=MojangTricksIntelDriversForPerformance_javaw.exe_minecraft.exe.heapdump -Djava.library.path={} -Dminecraft.launcher.brand=java-minecraft-launcher -Dminecraft.launcher.version=1.6.89-j -cp {}:{}/client.jar net.minecraft.client.main.Main --username {} --version '{} MCTui' --accessToken 0 --userProperties {{}} --gameDir {} --assetsDir {} --assetIndex {} --width 1280 --height 720",java, natives, libs, profile, username, version, game_dir, assets, version);

    let logs = Command::new("bash")
        .arg("-c")
        .arg(cmd)
        .output()
        .expect("failed to execute process");
}