mod utils;
mod structs;
mod constants;
mod mctui;

use std::path::Path;
use utils::*;
use structs::settings;
use constants::DOT_MCTUI;
use lazy_static::lazy_static;
use std::sync::Mutex;
use std::fs::create_dir_all;
use crate::mctui::tui::start_tui;

lazy_static! {
    static ref SETTINGS: Mutex<settings::Settings> = Mutex::new(settings::Settings::new().unwrap());
    static ref CONNECTION: Mutex<bool> = Mutex::new(false);
}

fn main() {
    create_dir_all(DOT_MCTUI).unwrap();
    std::env::set_current_dir(Path::new(DOT_MCTUI)).unwrap();
    universal::start_checker();
    start_tui();
//    universal::create_profile("test".to_string(), "1.13.2".to_string(), "1.13.1".to_string());
//    let settings = SETTINGS.lock().unwrap();
//    let selected = settings.profiles.selected.to_owned();
//    std::mem::drop(settings);
//
//    launch::prepare_game(&selected);
}
