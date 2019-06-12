mod utils;
mod structs;
mod constants;

use std::path::Path;
use utils::*;
use structs::settings;
use constants::DOT_MCTUI;
use lazy_static::lazy_static;
use std::sync::Mutex;

lazy_static! {
    static ref SETTINGS: Mutex<settings::Settings> = Mutex::new(settings::Settings::new().unwrap());
    static ref CONNECTION: Mutex<bool> = Mutex::new(false);
}

fn main() {
    std::env::set_current_dir(Path::new(DOT_MCTUI)).unwrap();
    universal::start_checker();
//    universal::create_profile("test".to_string(), "1.12.2".to_string());
    let settings = SETTINGS.lock().unwrap();
    let selected = settings.profiles.selected.to_owned();
    std::mem::drop(settings);

    launch::prepare_game(&selected);
}
