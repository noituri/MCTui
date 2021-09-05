mod constants;
mod launcher;
mod mctui;
mod structs;
mod utils;

use crate::mctui::tui::start_tui;
use launcher::Launcher;
use platform_dirs::AppDirs;
use std::fs::create_dir_all;
use std::sync::{Arc, Mutex};

type LauncherPtr = Arc<Mutex<Launcher>>;

#[tokio::main]
async fn main() {
    let app_dirs = AppDirs::new(Some("mctui"), false)
        .expect("Unable to get the platform Application Directories");

    let dot = &app_dirs.data_dir;

    create_dir_all(&dot.as_path()).expect("Unable to create the launcher application directory");

    let launcher = Launcher::new(app_dirs).expect("Unable to initialize the launcher");
    let launcher_ptr = Arc::new(Mutex::new(launcher));

    start_tui(launcher_ptr).await.unwrap();
}
