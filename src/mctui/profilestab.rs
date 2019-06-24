use super::app::{WinWidget, Window};
use crate::SETTINGS;
use termion::event::Key;
use tui::Frame;
use tui::layout::{Rect, Layout, Direction, Constraint};
use tui::backend::Backend;
use tui::widgets::{Table, Block, Widget, Row, Borders, Paragraph, Text};
use tui::style::{Style, Color, Modifier};

pub struct ProfilesTab {
    profiles_len: usize,
    selected_index: usize
}

impl WinWidget for ProfilesTab {
    fn new() -> Self {
        ProfilesTab {
            profiles_len: 0,
            selected_index: 0
        }
    }

    fn handle_events(&mut self, key: Key) -> Option<Window> {
        match key {
            Key::Char('\n') => {
                let settings = SETTINGS.lock().unwrap();
                return Some(Window::ProfileCreator(settings.profiles.profiles[self.selected_index].id.to_owned()))
            }
            Key::Char('n') => {
                return Some(Window::ProfileCreator(String::new()))
            }
            Key::Down => {
                if self.selected_index + 1 != self.profiles_len {
                    self.selected_index += 1;
                } else {
                    self.selected_index = 0;
                }
            }
            Key::Up => {
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

    fn render<B>(&mut self, backend: &mut Frame<B>, _: Option<Rect>) where B: Backend {
        let layout = Layout::default()
            .direction(Direction::Vertical)
            .margin(2)
            .constraints([Constraint::Length(3), Constraint::Max(14), Constraint::Max(1)].as_ref())
            .split(
                Layout::default().direction(Direction::Horizontal)
                    .constraints([
                        Constraint::Percentage(30),
                        Constraint::Percentage(40),
                        Constraint::Percentage(30)
                    ].as_ref()).split(backend.size())[1]);

        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .margin(1)
            .constraints([Constraint::Ratio(2, 3), Constraint::Ratio(1, 4)].as_ref())
            .split(layout[1]);


        Block::default().borders(Borders::ALL).title("Profiles").render(backend, layout[1]);

        let header = ["Name", "Version"];

        let mut settings = SETTINGS.lock().unwrap();
        self.profiles_len = settings.profiles.profiles.len();

        let rows = settings.profiles.profiles.iter().enumerate().map(|(i, item)| {
            if i == self.selected_index {
                Row::StyledData(vec![item.name.to_owned(), item.version.to_owned()].into_iter(), Style::default().fg(Color::Cyan))
            } else {
                Row::StyledData(vec![item.name.to_owned(), item.version.to_owned()].into_iter(), Style::default())
            }
        });

        Table::new(header.into_iter(), rows)
            .block(Block::default().borders(Borders::ALL))
            .widths(&[10, 10, 10])
            .render(backend, chunks[0]);

        Paragraph::new([
            Text::raw(" Press "),
            Text::styled("enter", Style::default().fg(Color::Cyan).modifier(Modifier::BOLD)),
            Text::raw(" to edit profile\n"),
            Text::raw(" Press "),
            Text::styled("n", Style::default().fg(Color::Cyan).modifier(Modifier::BOLD)),
            Text::raw(" to create new profile\n"),
            Text::raw(" Press "),
            Text::styled("d", Style::default().fg(Color::Cyan).modifier(Modifier::BOLD)),
            Text::raw(" to delete profile\n")
        ].iter())
            .wrap(true)
            .block(Block::default().borders(Borders::TOP))
            .render(backend, chunks[1]);
    }
}

