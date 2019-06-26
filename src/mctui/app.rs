use crate::structs::versions;
use crate::constants::VERSIONS;
use crate::SETTINGS;
use super::welcome::WelcomeWindow;
use super::home::HomeWindow;
use super::profilecreator::ProfileCreatorWindow;
use tui::Frame;
use tui::layout::Rect;
use tui::backend::Backend;
use termion::event::Key;

pub enum Window {
    Home(String),
    Welcome(String),
    ProfileCreator(String)
}

pub struct App {
    pub current_window: Window,
    pub windows: Windows<WelcomeWindow, HomeWindow, ProfileCreatorWindow>,
}

pub struct Windows<W, H, P> where W: WinWidget, H: WinWidget, P: WinWidget {
    pub welcome: W,
    pub home: H,
    pub profile_creator: P
}

impl App {
    pub fn new() -> App {
        let settings = SETTINGS.lock().unwrap();
        let mut current_window = Window::Welcome(String::new());

        if settings.auth.username != "" {
            current_window = Window::Home(String::new());
        }

        App {
            current_window,
            windows: Windows {
                welcome: WelcomeWindow::new(),
                home: HomeWindow::new(),
                profile_creator: ProfileCreatorWindow::new()
            }
        }
    }
}

pub trait WinWidget {
    fn new() -> Self;
    fn handle_events(&mut self, key: Key) -> Option<Window>;
    fn render<B>(&mut self, backend: &mut Frame<B>, _: Option<Rect>) where B: Backend;
}