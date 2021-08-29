use crate::structs::settings::Profile;
use crate::SettingsPtr;
use std::process::Command;
use uuid::Uuid;

pub async fn start_checker() {
    let arg = if cfg!(target_os = "windows") {
        "-n"
    } else {
        "-c"
    };

    let _output = Command::new("ping")
        .arg(arg)
        .arg("1")
        .arg("8.8.8.8")
        .output()
        .unwrap();
}

pub fn get_profile(id: &str, settings: SettingsPtr) -> Option<Profile> {
    let settings = settings.lock().unwrap();
    settings
        .profiles
        .profiles
        .iter()
        .find(|x| x.id == id)
        .map(Clone::clone)
}

pub fn create_profile(
    name: String,
    version: String,
    asset: String,
    args: String,
    settings: SettingsPtr,
) {
    let mut settings = settings.lock().unwrap();

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

    if settings.profiles.selected.is_empty() {
        settings.profiles.selected = id;
    }

    settings.save();
}

pub fn edit_profile(id: String, name: String, version: String, settings: SettingsPtr) {
    let mut settings = settings.lock().unwrap();

    for p in settings.profiles.profiles.iter_mut() {
        if p.id == id {
            p.name = name.to_owned();
            p.version = version.to_owned();
        }
    }

    settings.save();
}

pub fn delete_profile(id: String, settings: SettingsPtr) {
    let mut settings = settings.lock().unwrap();

    let index = settings
        .profiles
        .profiles
        .iter()
        .position(|p| *p.id == id)
        .unwrap();
    settings.profiles.profiles.remove(index);
    settings.save();
}
