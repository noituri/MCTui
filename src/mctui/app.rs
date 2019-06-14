use crate::structs::versions;
use crate::constants::VERSIONS;
use tui::widgets::Text;
use std::sync::mpsc::{Receiver, channel};

pub struct App<'a> {
    pub versions: Option<versions::Versions>,
    pub logs: Receiver<Vec<Text<'a>>>
}

impl<'a> App<'a> {
    pub fn new() -> App<'a> {
        let mut app = App {
            versions: None,
            logs: channel().1
        };

        if *crate::CONNECTION.lock().unwrap() {
            app.versions = Some(reqwest::get(VERSIONS).unwrap().json().unwrap());
        }

        app
    }
}