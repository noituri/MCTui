use crate::structs::settings::Profile;
use crate::SETTINGS;
use std::fs::File;
use std::process::Command;
use uuid::Uuid;

pub fn start_checker() {
    let settings = SETTINGS.lock().unwrap();

    let arg = if cfg!(target_os = "windows") {
        "-n"
    } else {
        "-c"
    };

    let output = Command::new("ping")
        .arg(arg)
        .arg("1")
        .arg("8.8.8.8")
        .output()
        .unwrap();

    *crate::CONNECTION.lock().unwrap() = output.status.success();

    if settings.auth.online {
        // TODO Yggdrasil
        panic!("implement me");
    }
}

pub fn get_profile(id: &str) -> Option<Profile> {
    for p in SETTINGS.lock().unwrap().profiles.profiles.iter() {
        if p.id == id {
            return Some(Profile {
                id: p.id.to_owned(),
                name: p.name.to_owned(),
                version: p.version.to_owned(),
                asset: p.asset.to_owned(),
                args: p.args.to_owned(),
            });
        }
    }

    None
}

pub fn create_profile(name: String, version: String, asset: String, args: String) {
    let mut settings = SETTINGS.lock().unwrap();

    let mut id = Uuid::new_v4().to_string();

    loop {
        let mut exists = false;
        for p in &settings.profiles.profiles {
            if p.id == id {
                id = Uuid::new_v4().to_string();
                exists = true
            }
        }

        if !exists {
            break;
        }
    }

    settings.profiles.profiles.push(Profile {
        id: id.to_owned(),
        name,
        version,
        asset,
        args,
    });

    if settings.profiles.selected == "" {
        settings.profiles.selected = id;
    }

    save_settings(&*settings);
}

pub fn edit_profile(id: String, name: String, version: String) {
    let mut settings = SETTINGS.lock().unwrap();

    for p in settings.profiles.profiles.iter_mut() {
        if p.id == id {
            p.name = name.to_owned();
            p.version = version.to_owned();
        }
    }

    save_settings(&*settings);
}

pub fn delete_profile(id: String) {
    let mut settings = SETTINGS.lock().unwrap();

    let index = settings
        .profiles
        .profiles
        .iter()
        .position(|p| *p.id == id)
        .unwrap();
    settings.profiles.profiles.remove(index);
    save_settings(&*settings);
}

pub fn save_settings(settings: &crate::settings::Settings) {
    serde_json::to_writer_pretty(
        &File::create(format!(
            "{}/mctui.json",
            std::env::var("DOT_MCTUI").unwrap()
        ))
        .unwrap(),
        settings,
    )
    .unwrap();
}
