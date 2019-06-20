use std::sync::Mutex;
use tui::backend::Backend;
use tui::layout::{Rect, Layout, Direction, Constraint};
use tui::Frame;
use tui::widgets::{Paragraph, Borders, Text, Block, Widget};
use tui::style::{Style, Color, Modifier};

//TODO trait

pub enum Selected {
    Username,
    Password
}

pub struct WelcomeWindow {
    pub input: (String, String),
    pub selected: Selected
}

impl WelcomeWindow {
    pub fn new() -> WelcomeWindow {
        WelcomeWindow {
            input: (String::new(), String::new()),
            selected: Selected::Username
        }
    }

    pub fn render<B>(&mut self, backend: &mut Frame<B>, rect: Rect) where B: Backend  {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .margin(1)
            .constraints([Constraint::Ratio(1,3), Constraint::Ratio(1,3), Constraint::Ratio(1,5)].as_ref())
            .split(rect);

        Block::default().borders(Borders::ALL).title("Sign In").render(backend, rect);

        Paragraph::new([Text::raw(self.input.0.to_owned())].iter())
            .block(Block::default()
                .borders(Borders::ALL)
                .border_style(match self.selected {
                    Selected::Username => Style::default().fg(Color::Cyan).modifier(Modifier::BOLD),
                    _ => Style::default().fg(Color::Black)
                })
                .title("Username or Email"))
            .render(backend, chunks[0]);

        let dotted_pass = "*".repeat(self.input.1.len());
        Paragraph::new([Text::raw(dotted_pass)].iter())
            .block(Block::default()
                .borders(Borders::ALL)
                .border_style(match self.selected {
                    Selected::Password => Style::default().fg(Color::Cyan).modifier(Modifier::BOLD),
                    _ => Style::default().fg(Color::Black)
                })
                .title("Password"))
            .render(backend, chunks[1]);

            let style = Style::default().fg(Color::Cyan).modifier(Modifier::BOLD);
            Paragraph::new([
                    Text::raw(" Press "),
                    Text::styled("enter", style),
                    Text::raw(" to submit"),
                    Text::raw("\n Leave password empty if you want to use offline mode")
                ]
                .iter())
                .wrap(true)
                .block(Block::default().borders(Borders::TOP))
                .render(backend, chunks[2]);
    }
}