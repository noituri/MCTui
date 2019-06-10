mod utils;
mod structs;
mod constants;

use std::path::Path;
use utils::*;
use constants::DOT_MCTUI;


fn main() {
    std::env::set_current_dir(Path::new(DOT_MCTUI));
    launch::prepare_game();
}
