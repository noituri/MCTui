use tui::backend::Backend;
use tui::layout::{Layout, Direction, Constraint, Rect};
use tui::Frame;
use super::logger::LoggerFrame;
use super::bottomnav::BottomNav;
use super::profilestab::ProfilesTab;
use super::app::{WinWidget, Window};
use crossbeam_channel::{Receiver, Sender};
use tui::widgets::{Tabs, Block, Widget, Borders};
use tui::style::{Style, Color};

pub struct HomeWindow {
    pub sender: Option<Sender<String>>,
    pub receiver: Option<Receiver<String>>,
    pub tab_index: usize,
    pub bottom_nav: BottomNav,
    pub profiles_tab: ProfilesTab,
    logger: LoggerFrame,
}

impl WinWidget for HomeWindow {
    fn new() -> HomeWindow {
        HomeWindow {
            sender: None,
            receiver: None,
            tab_index: 0,
            logger: LoggerFrame::new(),
            bottom_nav: BottomNav::new(),
            profiles_tab: ProfilesTab::new()
        }
    }

    fn handle_events(&mut self, key: Key) -> Option<Window> {
        match key {
            Key::Char('\t') => {
                self.tab_index = match self.tab_index {
                    0 => 1,
                    1 => 0,
                    _ => 0
                }
            }
            _ => {}
        }

        None
    }

    fn render<B>(&mut self, backend: &mut Frame<B>, _rect: Option<Rect>) where B: Backend {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Length(3), Constraint::Percentage(72), Constraint::Percentage(20)].as_ref())
            .split(backend.size());

        Tabs::default()
            .block(Block::default().borders(Borders::ALL).title("Tabs"))
            .titles(&vec!["Home", "Profiles"])
            .select(self.tab_index)
            .style(Style::default().fg(Color::Cyan))
            .highlight_style(Style::default().fg(Color::Yellow))
            .render(backend, chunks[0]);

        match self.tab_index {
            0 => {
                self.logger.receiver = self.receiver.to_owned();
                self.logger.render(backend, Some(chunks[1]));

                self.bottom_nav.sender = self.sender.to_owned();
                self.bottom_nav.render(backend, Some(chunks[2]));
            }
            1 => {
                self.profiles_tab.render(backend, None);
            }
            _ => {}
        }
    }
}