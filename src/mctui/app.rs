use crate::structs::versions;
use crate::constants::VERSIONS;
use super::welcome::WelcomeWindow;
use super::home::HomeWindow;

pub enum Window {
    Home,
    Welcome
}

pub struct App<'a> {
    pub versions: Option<versions::Versions>,
    pub current_window: Window,
    pub windows: Windows<'a>,
    pub logs: Vec<String>
}

pub struct Windows<'a> {
    pub welcome: WelcomeWindow,
    pub home: HomeWindow<'a>
}

impl<'a> App<'a> {
    pub fn new() -> App<'a> {
        let mut app = App {
            versions: None,
            current_window: Window::Home,
            windows: Windows {
                welcome: WelcomeWindow::new(),
                home: HomeWindow::new()
            },
            logs: Vec::new()
        };

        if *crate::CONNECTION.lock().unwrap() {
            app.versions = Some(reqwest::get(VERSIONS).unwrap().json().unwrap());
        }

        app
    }
}