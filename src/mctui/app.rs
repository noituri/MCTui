use crate::structs::versions;
use crate::constants::VERSIONS;
use super::welcome::WelcomeWindow;
use super::logger::LoggerFrame;
use tui::widgets::Text;
use std::sync::mpsc::{Receiver, channel, Sender};

pub enum Window {
    Home,
    Welcome
}

pub struct App {
    pub versions: Option<versions::Versions>,
    pub current_window: Window,
    pub windows: Windows,
    pub logs: Vec<String>
}

pub struct Windows {
    pub welcome: WelcomeWindow,
    pub home: LoggerFrame
}

impl App {
    pub fn new() -> App {
        let mut app = App {
            versions: None,
            current_window: Window::Welcome,
            windows: Windows {
                welcome: WelcomeWindow::new(),
                home: LoggerFrame::new()
            },
            logs: Vec::new()
        };

        if *crate::CONNECTION.lock().unwrap() {
            app.versions = Some(reqwest::get(VERSIONS).unwrap().json().unwrap());
        }

        app
    }
}