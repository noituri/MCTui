use crate::structs::versions;
use crate::constants::VERSIONS;
use super::welcome::WelcomeWindow;
use super::home::HomeWindow;
use super::profilecreator::ProfileCreatorWindow;
use tui::Frame;
use tui::layout::Rect;
use tui::backend::Backend;

pub enum Window {
    Home,
    Welcome,
    ProfileCreator
}

pub struct App<'a> {
    pub versions: Option<versions::Versions>,
    pub current_window: Window,
    pub windows: Windows<WelcomeWindow, HomeWindow<'a>, ProfileCreatorWindow>,
    pub logs: Vec<String>
}

pub struct Windows<W, H, P> where W: WinWidget, H: WinWidget, P: WinWidget {
    pub welcome: W,
    pub home: H,
    pub profile_creator: P
}

impl<'a> App<'a> {
    pub fn new() -> App<'a> {
        let mut app = App {
            versions: None,
            current_window: Window::ProfileCreator,
            windows: Windows {
                welcome: WelcomeWindow::new(),
                home: HomeWindow::new(),
                profile_creator: ProfileCreatorWindow::new()
            },
            logs: Vec::new()
        };

        if *crate::CONNECTION.lock().unwrap() {
            app.versions = Some(reqwest::get(VERSIONS).unwrap().json().unwrap());
        }

        app
    }
}

pub trait WinWidget {
    fn new() -> Self;
    fn render<B>(&mut self, _: &mut Frame<B>, _: Option<Rect>) where B: Backend;
}