mod constants;
mod mctui;
mod structs;
mod utils;

use crate::mctui::tui::start_tui;
use platform_dirs::AppDirs;
use std::fs::create_dir_all;
use std::path::Path;
use std::sync::{Arc, Mutex};
use structs::settings::Settings;
use utils::*;

type SettingsPtr = Arc<Mutex<Settings>>;

#[tokio::main]
async fn main() {
    let mut dot = {
        let app_dirs = AppDirs::new(Some("mctui"), false).unwrap();
        app_dirs
            .data_dir
            .into_os_string()
            .into_string()
            .expect("hmmm, ah yes paths")
    };
    match std::env::var("DOT_MCTUI") {
        Ok(val) => dot = val,
        Err(_) => std::env::set_var("DOT_MCTUI", dot.to_owned()),
    }

    let settings = Settings::new().expect("Unable to initialize the application settings");
    let settings_ptr = Arc::new(Mutex::new(settings));

    create_dir_all(dot.to_owned()).unwrap();
    std::env::set_current_dir(Path::new(&dot)).unwrap();
    universal::start_checker().await;
    start_tui(settings_ptr).await.unwrap();
}
