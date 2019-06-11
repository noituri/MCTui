mod utils;
mod structs;
mod constants;

use std::path::Path;
use utils::*;
use structs::settings;
use constants::DOT_MCTUI;
use lazy_static::lazy_static;
use std::fs::File;
use serde::{Deserialize, Serialize};
use std::sync::Mutex;

lazy_static! {
    static ref SETTINGS: Mutex<settings::Settings> = Mutex::new(settings::Settings::new().unwrap());
}

fn main() {
    std::env::set_current_dir(Path::new(DOT_MCTUI));
    universal::start_checker();
//    serde_json::to_writer_pretty(&File::create("test.json").unwrap(), &*SETTINGS);
    launch::prepare_game();
}
