use crate::structs::versions;
use crate::constants::VERSIONS;
use tui::widgets::Text;
use std::sync::mpsc::{Receiver, channel, Sender};

pub struct App {
    pub versions: Option<versions::Versions>,
    pub logs: Vec<String>
}

impl App {
    pub fn new() -> App {
        let mut app = App {
            versions: None,
            logs: Vec::new()
        };

        if *crate::CONNECTION.lock().unwrap() {
            app.versions = Some(reqwest::get(VERSIONS).unwrap().json().unwrap());
        }

        app
    }
}