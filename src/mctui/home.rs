use tui::backend::Backend;
use tui::layout::{Layout, Direction, Constraint, Rect};
use tui::Frame;
use super::logger::LoggerFrame;
use super::bottomnav::BottomNav;
use super::app::WinWidget;
use crossbeam_channel::Receiver;

pub struct HomeWindow<'a> {
    pub receiver: Option<Receiver<String>>,
    logger: LoggerFrame,
    bottom_nav: BottomNav<'a>
}

impl<'a> WinWidget for HomeWindow<'a> {
    fn new() -> HomeWindow<'a> {
        HomeWindow {
            receiver: None,
            logger: LoggerFrame::new(),
            bottom_nav: BottomNav::new()
        }
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