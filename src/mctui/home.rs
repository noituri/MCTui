use tui::backend::Backend;
use tui::layout::{Layout, Direction, Constraint, Rect};
use tui::Frame;
use super::logger::LoggerFrame;
use super::bottomnav::BottomNav;
use super::app::{WinWidget, Window};
use crossbeam_channel::{Receiver, Sender};
use termion::event::Key;
use std::thread;

pub struct HomeWindow<'a> {
    pub sender: Option<Sender<String>>,
    pub receiver: Option<Receiver<String>>,
    logger: LoggerFrame,
    bottom_nav: BottomNav<'a>,
}

impl<'a> WinWidget for HomeWindow<'a> {
    fn new() -> HomeWindow<'a> {
        HomeWindow {
            sender: None,
            receiver: None,
            logger: LoggerFrame::new(),
            bottom_nav: BottomNav::new(),
        }
    }

    fn handle_events(&mut self, key: Key) -> Option<Window> {
        match key {
            Key::Char('\n') => {
                let settings = crate::SETTINGS.lock().unwrap();
                let selected = settings.profiles.selected.to_owned();
                std::mem::drop(settings);

                match self.sender.to_owned() {
                    Some(sender) => {
                        thread::spawn(move || {
                            crate::utils::launch::prepare_game(&selected, sender);
                        });
                    },
                    None => {}
                }
            }
            _ => {}
        }

        None
    }

    fn render<B>(&mut self, backend: &mut Frame<B>, _rect: Option<Rect>) where B: Backend {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Percentage(80), Constraint::Percentage(20)].as_ref())
            .split(backend.size());

        self.logger.receiver = self.receiver.to_owned();
        self.logger.render(backend, Some(chunks[0]));
        self.bottom_nav.render(backend, Some(chunks[1]));
    }
}