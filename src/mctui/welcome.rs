use tui::backend::Backend;
use tui::layout::{Layout, Direction, Constraint, Rect};
use tui::Frame;
use tui::widgets::{Paragraph, Borders, Text, Block, Widget};
use tui::style::{Style, Color, Modifier};
use super::app::WinWidget;
use termion::event::Key;
use super::app::Window;
use crate::SETTINGS;
use crate::universal::save_settings;

pub enum Selected {
    Username,
    Password,
}

pub struct WelcomeWindow {
    pub input: (String, String),
    pub selected: Selected,
}

impl WinWidget for WelcomeWindow {
    fn new() -> WelcomeWindow {
        WelcomeWindow {
            input: (String::new(), String::new()),
            selected: Selected::Username,
        }
    }

    fn handle_events(&mut self, key: Key) -> Option<Window> {
        match key {
            Key::Char('\n') => {
                let mut settings = SETTINGS.lock().unwrap();
                settings.auth.username = self.input.0.to_owned();
                save_settings(&*settings);

                if settings.profiles.profiles.len() == 0 {
                    return Some(Window::ProfileCreator(String::new()));
                } else {
                    return Some(Window::Home(String::new()));
                }
            }
            Key::Down | Key::Up | Key::Char('\t') => {
                match self.selected {
                    Selected::Username => self.selected = Selected::Password,
                    Selected::Password => self.selected = Selected::Username
                }
            }
            Key::Backspace => {
                match self.selected {
                    Selected::Username => {
                        self.input.0.pop();
                    }
                    Selected::Password => {
                        self.input.1.pop();
                    }
                };
            }
            Key::Char(ch) => {
                match self.selected {
                    Selected::Username => self.input.0.push(ch),
                    Selected::Password => self.input.1.push(ch)
                }
            }
            _ => {}
        }
        None
    }

    fn render<B>(&mut self, backend: &mut Frame<B>, _rect: Option<Rect>) where B: Backend {
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
            .constraints([Constraint::Ratio(1, 4), Constraint::Ratio(1, 4), Constraint::Ratio(1, 4)].as_ref())
            .split(layout[1]);

        Block::default().borders(Borders::ALL).title("Sign In").render(backend, layout[1]);

        Paragraph::new([Text::raw(self.input.0.to_owned())].iter())
            .block(Block::default()
                .borders(Borders::ALL)
                .border_style(match self.selected {
                    Selected::Username => Style::default().fg(Color::Cyan).modifier(Modifier::BOLD),
                    _ => Style::default()
                })
                .title("Username or Email"))
            .render(backend, chunks[0]);

        let dotted_pass = "*".repeat(self.input.1.len());
        Paragraph::new([Text::raw(dotted_pass)].iter())
            .block(Block::default()
                .borders(Borders::ALL)
                .border_style(match self.selected {
                    Selected::Password => Style::default().fg(Color::Cyan).modifier(Modifier::BOLD),
                    _ => Style::default()
                })
                .title("Password"))
            .render(backend, chunks[1]);

        let style = Style::default().fg(Color::Cyan).modifier(Modifier::BOLD);
        Paragraph::new([
            Text::raw(" Press "),
            Text::styled("enter", style),
            Text::raw(" to submit"),
            Text::raw("\n Leave password empty if you want to use offline mode (online mode is not working right now)")
        ]
            .iter())
            .wrap(true)
            .block(Block::default().borders(Borders::TOP))
            .render(backend, chunks[2]);
    }
}