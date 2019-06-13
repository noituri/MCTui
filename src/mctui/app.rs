use crate::structs::versions;
use crate::constants::VERSIONS;

pub struct App {
    pub versions: Option<versions::Versions>,
}

impl App {
    pub fn new() -> App {
        let mut app = App {
            versions: None,
        };

        if *crate::CONNECTION.lock().unwrap() {
            app.versions = Some(reqwest::get(VERSIONS).unwrap().json().unwrap());
        }

        app
    }
}