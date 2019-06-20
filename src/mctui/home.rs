use tui::backend::Backend;
use tui::layout::{Layout, Direction, Constraint};
use tui::Frame;
use super::logger::LoggerFrame;
use super::bottomnav::BottomNav;
use crossbeam_channel::Receiver;

pub struct HomeWindow<'a> {
    logger: LoggerFrame,
    bottom_nav: BottomNav<'a>
}

impl<'a> HomeWindow<'a> {
    pub fn new() -> HomeWindow<'a> {
        HomeWindow {
            logger: LoggerFrame::new(),
            bottom_nav: BottomNav::new()
        }
    }

    pub fn render<B>(&mut self, backend: &mut Frame<B>, receiver: Receiver<String>) where B: Backend {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Percentage(80), Constraint::Percentage(20)].as_ref())
            .split(backend.size());

        self.logger.render(backend, chunks[0], receiver.clone());
        self.bottom_nav.render(backend, chunks[1]);
    }
}