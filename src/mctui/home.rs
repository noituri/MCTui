use async_trait::async_trait;
use crossterm::event::KeyCode;
use tui::layout::{Constraint, Direction, Layout, Rect};
use tui::style::{Color, Style};
use tui::widgets::{Block, Borders, Tabs};
use tui::Frame;
use tui::{backend::Backend, text::Spans};

use crate::SettingsPtr;

use super::{
    app::{TuiWidget, WindowType},
    bottomnav::BottomNav,
    logger::LoggerFrame,
    profilestab::ProfilesTab,
};

pub struct HomeWindow {
    pub tab_index: usize,
    pub bottom_nav: BottomNav,
    pub profiles_tab: ProfilesTab,
    pub logger: LoggerFrame,
}

impl HomeWindow {
    pub fn new(settings: SettingsPtr) -> Self {
        Self {
            tab_index: 0,
            logger: LoggerFrame::new(),
            bottom_nav: BottomNav::new(settings.clone()),
            profiles_tab: ProfilesTab::new(settings.clone()),
        }
    }
}

#[async_trait]
impl TuiWidget for HomeWindow {
    async fn handle_events(&mut self, key: KeyCode) -> Option<WindowType> {
        if self.tab_index == 0 {
            self.bottom_nav.handle_events(key).await;
        } else {
            let result = self.profiles_tab.handle_events(key).await;
            if result.is_some() {
                return result;
            }
        }
        if let KeyCode::Tab = key {
            self.tab_index = match self.tab_index {
                0 => 1,
                1 => 0,
                _ => 0,
            }
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
                self.logger.render(frame, Some(chunks[1]));
                self.bottom_nav.render(frame, Some(chunks[2]));
            }
            1 => {
                self.profiles_tab.render(frame, None);
            }
            _ => {}
        }
    }
}
