use reqwest;
use reqwest::StatusCode;
use std::fs;
use std::fs::File;
use std::fs::create_dir_all;
use std::io;
use std::path::Path;
use std::process::Command;

pub fn download_file(url: String, path: &str) {
    create_dir_all(path);

    let url_parts: Vec<&str> = url.split('/').collect();
    let output = Path::new(path).join(url_parts.last().unwrap());

    match reqwest::get(url.as_str()) {
        Ok(mut resp) => {
            match resp.status() {
                StatusCode::OK => (),
                _ => {
                    println!("Could not download this file: {}", url);
                    return;
                },
            }
            let mut file = match File::create(&output) {
                Ok(f) => f,
                Err(err) => {
                    println!("Error occurred while creating file: {} | Error: {}", output.display(), err);
                    return;
                }
            };
            match io::copy(&mut resp, &mut file) {
                Ok(_) => println!("File {} has been downloaded", output.display()),
                Err(err) => println!("Could not download this file: {} | Error: {}", url, err),
            }
        },

        Err(err) => println!("Could not download this file: {} | Error: {}", url, err),
    };
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

    let cmd = format!("{} -Xmx1G -XX:+UseConcMarkSweepGC -XX:+CMSIncrementalMode -XX:-UseAdaptiveSizePolicy -Xmn128M -XX:HeapDumpPath=MojangTricksIntelDriversForPerformance_javaw.exe_minecraft.exe.heapdump -Djava.library.path={} -Dminecraft.launcher.brand=java-minecraft-launcher -Dminecraft.launcher.version=1.6.89-j -cp {}:{}/client.jar net.minecraft.client.main.Main --username {} --version '{} MCTui' --accessToken 0 --userProperties {{}} --gameDir {} --assetsDir {} --assetIndex {} --width 1280 --height 720",java, natives, libs, profile, username, version, game_dir, assets, version);

    let logs = Command::new("bash")
        .arg("-c")
        .arg(cmd)
        .output()
        .expect("failed to execute process");
}
