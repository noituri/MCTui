use crate::SETTINGS;
use crate::constants::DOT_MCTUI;
use std::path::Path;

pub fn start_checker() {
    let mut settings = SETTINGS.lock().unwrap();

    if settings.auth.username == "" && !settings.auth.online {
        settings.auth.username = "Steve".to_string();
    }

    if settings.auth.online {
        // TODO Yggdrasil
        panic!("implement me");
    }
}