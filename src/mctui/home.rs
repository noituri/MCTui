use crossbeam_channel::{Receiver, Sender};
use crossterm::event::KeyCode;
use tui::layout::{Constraint, Direction, Layout, Rect};
use tui::style::{Color, Style};
use tui::widgets::{Block, Borders, Tabs};
use tui::Frame;
use tui::{backend::Backend, text::Spans};
use async_trait::async_trait;

use super::{
    app::{TuiWidget, WindowType},
    bottomnav::BottomNav,
    logger::LoggerFrame,
    profilestab::ProfilesTab,
};

pub struct HomeWindow {
    pub sender: Option<Sender<String>>,
    pub receiver: Option<Receiver<String>>,
    pub tab_index: usize,
    pub bottom_nav: BottomNav,
    pub profiles_tab: ProfilesTab,
    logger: LoggerFrame,
}

impl HomeWindow {
    pub fn new() -> Self {
        Self {
            sender: None,
            receiver: None,
            tab_index: 0,
            logger: LoggerFrame::new(),
            bottom_nav: BottomNav::new(),
            profiles_tab: ProfilesTab::new(),
        }
    }
}

#[async_trait]
impl TuiWidget for HomeWindow {
    async fn handle_events(&mut self, key: KeyCode) -> Option<WindowType> {
        if self.tab_index == 0 {
            self.bottom_nav.handle_events(key);
        } else {
            let result = self.profiles_tab.handle_events(key).await;
            if result.is_some() {
                return result;
            }
        }
        match key {
            KeyCode::Tab => {
                self.tab_index = match self.tab_index {
                    0 => 1,
                    1 => 0,
                    _ => 0,
                }
            }
            _ => {}
        }

        None
    }

    fn render<B>(&mut self, frame: &mut Frame<B>, _: Option<Rect>)
    where
        B: Backend,
    {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints(
                [
                    Constraint::Length(3),
                    Constraint::Percentage(72),
                    Constraint::Percentage(20),
                ]
                .as_ref(),
            )
            .split(frame.size());

        let titles = ["Home", "Profiles"]
            .iter()
            .cloned()
            .map(Spans::from)
            .collect();
        let tabs = Tabs::new(titles)
            .block(Block::default().borders(Borders::ALL).title("Tabs"))
            .select(self.tab_index)
            .style(Style::default().fg(Color::Cyan))
            .highlight_style(Style::default().fg(Color::Yellow));
        frame.render_widget(tabs, chunks[0]);

        match self.tab_index {
            0 => {
                self.logger.receiver = self.receiver.to_owned();
                self.logger.render(frame, Some(chunks[1]));

                self.bottom_nav.sender = self.sender.to_owned();
                self.bottom_nav.render(frame, Some(chunks[2]));
            }
            1 => {
                self.profiles_tab.render(frame, None);
            }
            _ => {}
        }
    }
}
