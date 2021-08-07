use crate::{utils::universal::delete_profile, SETTINGS};
use async_trait::async_trait;
use crossterm::event::KeyCode;
use tui::layout::{Constraint, Direction, Layout, Rect};
use tui::style::{Color, Modifier, Style};
use tui::text::{Span, Spans};
use tui::Frame;
use tui::{
    backend::Backend,
    widgets::{Block, Borders, Paragraph, Row, Table, Wrap},
};

use super::app::{TuiWidget, WindowType};

pub struct ProfilesTab {
    profiles_len: usize,
    selected_index: usize,
}

impl ProfilesTab {
    pub fn new() -> Self {
        Self {
            profiles_len: 0,
            selected_index: 0,
        }
    }
}

#[async_trait]
impl TuiWidget for ProfilesTab {
    async fn handle_events(&mut self, key: KeyCode) -> Option<WindowType> {
        match key {
            KeyCode::Enter => {
                let settings = SETTINGS.lock().unwrap();
                return Some(WindowType::ProfileCreator(
                    settings.profiles.profiles[self.selected_index]
                        .id
                        .to_owned(),
                ));
            }
            KeyCode::Char('n') => return Some(WindowType::ProfileCreator(String::new())),
            KeyCode::Char('d') => {
                let settings = SETTINGS.lock().unwrap();
                let id = settings.profiles.profiles[self.selected_index]
                    .id
                    .to_owned();
                std::mem::drop(settings);

                delete_profile(id);
            }
            KeyCode::Down => {
                if self.selected_index + 1 != self.profiles_len {
                    self.selected_index += 1;
                } else {
                    self.selected_index = 0;
                }
            }
            KeyCode::Up => {
                if self.selected_index > 0 {
                    self.selected_index -= 1;
                } else {
                    self.selected_index = self.profiles_len - 1;
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
        let layout = Layout::default()
            .direction(Direction::Vertical)
            .margin(2)
            .constraints(
                [
                    Constraint::Length(3),
                    Constraint::Max(14),
                    Constraint::Max(1),
                ]
                .as_ref(),
            )
            .split(
                Layout::default()
                    .direction(Direction::Horizontal)
                    .constraints(
                        [
                            Constraint::Percentage(30),
                            Constraint::Percentage(40),
                            Constraint::Percentage(30),
                        ]
                        .as_ref(),
                    )
                    .split(frame.size())[1],
            );

        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .margin(1)
            .constraints([Constraint::Ratio(2, 3), Constraint::Ratio(1, 4)].as_ref())
            .split(layout[1]);

        let block = Block::default().borders(Borders::ALL).title("Profiles");
        frame.render_widget(block, layout[1]);

        let header = Row::new(vec!["Name", "Version"]);
        let settings = SETTINGS.lock().unwrap();
        self.profiles_len = settings.profiles.profiles.len();

        let selected_style = Style::default().fg(Color::Cyan);
        let rows = settings
            .profiles
            .profiles
            .iter()
            .enumerate()
            .map(|(i, item)| {
                let mut row = Row::new(vec![item.name.to_owned(), item.version.to_owned()]);
                if i == self.selected_index {
                    row = row.style(selected_style);
                }
                row
            });

        let table = Table::new(rows)
            .header(header)
            .block(Block::default().borders(Borders::ALL))
            .widths(&[
                Constraint::Length(10),
                Constraint::Length(10),
                Constraint::Length(10),
                Constraint::Length(10),
            ]);
        frame.render_widget(table, chunks[0]);

        let style = Style::default()
            .fg(Color::Cyan)
            .add_modifier(Modifier::BOLD);
        let spans = Spans::from(vec![
            " Press ".into(),
            Span::styled("enter", style),
            " to edit profile\n".into(),
            " Press ".into(),
            Span::styled("n", style),
            " to create new profile\n".into(),
            " Press ".into(),
            Span::styled("d", style),
            " to delete profile\n".into(),
        ]);
        let paragraph = Paragraph::new(spans)
            .wrap(Wrap { trim: true })
            .block(Block::default().borders(Borders::TOP));
        frame.render_widget(paragraph, chunks[1]);
    }
}
