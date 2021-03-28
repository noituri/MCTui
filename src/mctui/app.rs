use std::{collections::HashMap, io::Stdout, slice::Windows};

use crate::SETTINGS;
use super::welcome::WelcomeWindow;
use crossterm::event::KeyCode;
// use super::home::HomeWindow;
// use super::profilecreator::ProfileCreatorWindow;
use tui::{Frame, widgets::Widget};
use tui::layout::Rect;
use tui::backend::Backend;

pub enum WindowType {
    Home,
    Welcome,
    ProfileCreator(String)
}

pub struct App {
    pub current_window: WindowType,
    pub windows: TuiWindows
}

pub struct TuiWindows {
    pub welcome: WelcomeWindow
}

impl App {
    pub fn new() -> Self {
        let settings = SETTINGS.lock().unwrap();
        let mut current_window = WindowType::Welcome;

        // if settings.auth.username != "" {
        //     current_window = Window::Home();
        // }
        Self {
            current_window, 
            windows: TuiWindows {
                welcome: WelcomeWindow::new()
            }
        }
    }
    pub fn render<B>(&mut self, frame: &mut Frame<B>)
    where
        B: Backend
    {
        match self.current_window {
            WindowType::Welcome => self.windows.welcome.render(frame, None),
            _ => unimplemented!()
        }
    }
}

pub trait TuiWidget {
    fn handle_events(&mut self, key: KeyCode) -> Option<WindowType>;
    fn render<B>(&mut self, frame: &mut Frame<B>, _: Option<Rect>) where B: Backend;
}