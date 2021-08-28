mod constants;
mod mctui;
mod structs;
mod utils;

use crate::mctui::tui::start_tui;
use platform_dirs::AppDirs;
use std::env;
use std::fs::create_dir_all;
use std::sync::{Arc, Mutex};
use structs::settings::Settings;
use utils::*;

type SettingsPtr = Arc<Mutex<Settings>>;

#[tokio::main]
async fn main() {
    let app_dirs = AppDirs::new(Some("mctui"), false)
        .expect("Unable to get the platform Application Directories");

    let dot = &app_dirs.data_dir;

    create_dir_all(&dot.as_path())
        .expect("Unable to create the launcher application directory");

    env::set_current_dir(&dot.as_path())
        .expect("Unable to change the current directory");

    let settings = Settings::new(app_dirs)
        .expect("Unable to initialize the application settings");
    let settings_ptr = Arc::new(Mutex::new(settings));

    universal::start_checker().await;
    start_tui(settings_ptr).await.unwrap();
}
