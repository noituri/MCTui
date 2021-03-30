use std::{collections::HashMap, io::Stdout, slice::Windows};

use crate::SETTINGS;
use super::{profilecreator::ProfileCreatorWindow, welcome::WelcomeWindow};
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
    pub welcome: WelcomeWindow,
    pub profile_creator: ProfileCreatorWindow
}

impl App {
    pub fn new() -> Self {
        let settings = SETTINGS.lock().unwrap();
        let mut current_window = WindowType::ProfileCreator(String::new());

        // if settings.auth.username != "" {
        //     current_window = Window::Home();
        // }
        Self {
            current_window, 
            windows: TuiWindows {
                welcome: WelcomeWindow::new(),
                profile_creator: ProfileCreatorWindow::new()
            }
        }
    }
    pub fn render<B>(&mut self, frame: &mut Frame<B>)
    where
        B: Backend
    {
        match self.current_window {
            WindowType::Welcome => self.windows.welcome.render(frame, None),
            WindowType::ProfileCreator(_) => self.windows.profile_creator.render(frame, None),
            _ => unimplemented!()
        }
    }

    pub fn handle_events(&mut self, key: KeyCode) {
        let window_route = match self.current_window {
            WindowType::Welcome => self.windows.welcome.handle_events(key),
            WindowType::ProfileCreator(_) => self.windows.profile_creator.handle_events(key),
            _ => unimplemented!()
        };

        if let Some(route) = window_route {
            self.current_window = route;
        }
    }
}

pub trait TuiWidget {
    fn handle_events(&mut self, key: KeyCode) -> Option<WindowType>;
    fn render<B>(&mut self, frame: &mut Frame<B>, _: Option<Rect>) where B: Backend;
}