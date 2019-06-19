use std::sync::Mutex;
use tui::backend::Backend;
use tui::layout::{Rect, Layout, Direction, Constraint};
use tui::Frame;
use tui::widgets::{Paragraph, Borders, Text, Block, Widget, SelectableList};
use tui::style::{Style, Color, Modifier};

//TODO trait

pub struct WelcomeWindow {
    pub input: (String, String)
}

impl WelcomeWindow {
    pub fn new() -> WelcomeWindow {
        WelcomeWindow {
            input: (String::new(), String::new())
        }
    }

    pub fn render<B>(&mut self, backend: &mut Frame<B>, rect: Rect) where B: Backend  {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .margin(1)
            .constraints([Constraint::Percentage(40), Constraint::Percentage(40), Constraint::Percentage(15)].as_ref())
            .split(rect);

        Block::default().borders(Borders::ALL).title("Sign In").render(backend, rect);

        Paragraph::new([Text::raw(self.input.0.to_owned())].iter())
            .block(Block::default()
                .borders(Borders::ALL)
                .title("Username or Email"))
            .render(backend, chunks[0]);

        let dotted_pass = "*".repeat(self.input.1.len());
        Paragraph::new([Text::raw(dotted_pass)].iter())
            .block(Block::default()
                .borders(Borders::ALL)
                .title("Password"))
            .render(backend, chunks[1]);

            let style = Style::default().fg(Color::Black).bg(Color::White);
            SelectableList::default()
                .block(Block::default().borders(Borders::TOP))
                .items(&vec!("Submit"))
                .select(Some(0))
                .highlight_style(style.fg(Color::LightGreen).modifier(Modifier::BOLD))
                .render(backend, chunks[2]);
    }
}