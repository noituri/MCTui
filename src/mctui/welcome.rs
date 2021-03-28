use tui::{backend::Backend, widgets::Wrap};
use tui::layout::{Layout, Direction, Constraint, Rect};
use tui::Frame;
use tui::widgets::{Paragraph, Borders, Block, Widget};
use tui::text::{Spans, Span};
use tui::style::{Style, Color, Modifier};
use super::app::TuiWidget;
use super::app::WindowType;
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

impl WelcomeWindow {
    pub fn new() -> WelcomeWindow {
        WelcomeWindow {
            input: (String::new(), String::new()),
            selected: Selected::Username,
        }
    }
}

impl TuiWidget for WelcomeWindow {
    fn handle_events(&mut self, key: Key) -> Option<WindowType> {
        match key {
            Key::Char('\n') => {
                let mut settings = SETTINGS.lock().unwrap();
                settings.auth.username = self.input.0.to_owned();
                save_settings(&*settings);

                if settings.profiles.profiles.len() == 0 {
                    return Some(WindowType::ProfileCreator(String::new()));
                } else {
                    return Some(WindowType::Home);
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

    fn render<B>(&mut self, frame: &mut Frame<B>, _: Option<Rect>)
    where
        B: Backend
    {
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
                    ].as_ref()).split(frame.size())[1]);

        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .margin(1)
            .constraints([Constraint::Ratio(1, 4), Constraint::Ratio(1, 4), Constraint::Ratio(1, 4)].as_ref())
            .split(layout[1]);

        let block = Block::default()
            .borders(Borders::ALL)
            .title("Sign In");
        frame.render_widget(block, layout[1]);

        let paragrapth = Paragraph::new(Spans::from(self.input.0.to_owned()))
            .block(Block::default()
                .borders(Borders::ALL)
                .border_style(match self.selected {
                    Selected::Username => Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD),
                    _ => Style::default()
                })
                .title("Username or Email"));
        frame.render_widget(paragrapth, chunks[0]);

        let dotted_pass = "*".repeat(self.input.1.len());
        let paragraph = Paragraph::new(Spans::from(dotted_pass))
            .block(Block::default()
                .borders(Borders::ALL)
                .border_style(match self.selected {
                    Selected::Password => Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD),
                    _ => Style::default()
                })
                .title("Password"));
        frame.render_widget(paragraph, chunks[1]);
    
        let style = Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD);
        let paragraph = Paragraph::new(vec![
            Spans::from(" Press "),
            Spans::from(Span::styled("enter", style)),
            Spans::from(" to submit"),
            Spans::from("\n Leave password empty if you want to use offline mode (online mode is not working right now)")
        ])
            .wrap(Wrap { trim: true })
            .block(Block::default().borders(Borders::TOP));
        frame.render_widget(paragraph, chunks[2]);
    }
}