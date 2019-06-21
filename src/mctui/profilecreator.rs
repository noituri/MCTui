use super::app::WinWidget;
use crate::structs::versions;
use crate::constants::VERSIONS;
use tui::Frame;
use tui::layout::{Rect, Layout, Direction, Constraint};
use tui::backend::Backend;
use tui::widgets::{Paragraph, Text, Block, Widget, Borders, SelectableList};
use tui::style::{Style, Color, Modifier};

pub struct ProfileCreatorWindow {
    pub input_name: String,
    pub selected_version: usize,
    pub versions: Vec<versions::Version>
}

impl WinWidget for ProfileCreatorWindow {
    fn new() -> ProfileCreatorWindow {
        let versions_resp: versions::Versions = reqwest::get(VERSIONS).unwrap().json().unwrap();

        ProfileCreatorWindow {
            input_name: String::new(),
            selected_version: 0,
            versions: versions_resp.versions,
        }
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
            .constraints([Constraint::Ratio(1,4), Constraint::Ratio(1,2), Constraint::Ratio(1,4)].as_ref())
            .split(layout[1]);

        Block::default().borders(Borders::ALL).title("Profile creator").render(backend, layout[1]);

        Paragraph::new([Text::raw(self.input_name.to_owned())].iter())
            .block(Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::Cyan).modifier(Modifier::BOLD))
            .title("Name"))
            .render(backend, chunks[0]);

        let versions: Vec<String> = self.versions.iter().map(|v| v.id.to_owned()).collect();

        SelectableList::default()
            .block(Block::default().borders(Borders::ALL).title("Options"))
            .items(&versions)
            .select(Some(self.selected_version))
            .highlight_style(Style::default().fg(Color::LightGreen).modifier(Modifier::BOLD))
            .highlight_symbol(">")
            .render(backend, chunks[1]);

        Paragraph::new([
            Text::raw(" Press "),
            Text::styled("enter", Style::default().fg(Color::Cyan).modifier(Modifier::BOLD)),
            Text::raw(" to submit")
        ].iter())
            .wrap(true)
            .block(Block::default().borders(Borders::TOP))
            .render(backend, chunks[2]);
    }
}