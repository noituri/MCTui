use crate::structs::versions;
use crate::constants::VERSIONS;
use crate::structs::libraries::Libraries;
use crossterm::event::KeyCode;
use tui::{Frame, text::{Span, Spans}, widgets::{List, ListItem, ListState, Wrap}};
use tui::layout::{Rect, Layout, Direction, Constraint};
use tui::backend::Backend;
use tui::widgets::{Paragraph, Block, Widget, Borders};
use tui::style::{Style, Color, Modifier};
use super::app::{TuiWidget, WindowType};

pub struct ProfileCreatorWindow {
    pub input: String,
    pub id: Option<String>,
    pub selected_version: usize,
    pub versions: Vec<versions::Version>,
    list_state: ListState
}

impl ProfileCreatorWindow {
    pub fn new() -> ProfileCreatorWindow {
        let versions_resp: versions::Versions = reqwest::get(VERSIONS).unwrap().json().unwrap();

        ProfileCreatorWindow {
            input: String::new(),
            id: None,
            selected_version: 0,
            versions: versions_resp.versions,
            list_state: ListState::default()
        }
    }
}

impl TuiWidget for ProfileCreatorWindow {
    fn handle_events(&mut self, key: KeyCode) -> Option<WindowType> {
        match key {
            KeyCode::Enter => {
                let selected_version = &self.versions[self.selected_version];
                //TODO check connection
                let assets_resp: Libraries = reqwest::get(selected_version.url.as_str()).unwrap().json().unwrap();

                match self.id.to_owned() {
                    Some(id) => {
                        crate::universal::edit_profile(
                          id,
                            self.input.to_owned(),
                          selected_version.id.to_owned(),
                        );
                    },
                    None => {
                        crate::universal::create_profile(
                            self.input.to_owned(),
                            selected_version.id.to_owned(),
                            assets_resp.asset_index.id,
                            "-Xmx1G -XX:+UseConcMarkSweepGC -XX:+CMSIncrementalMode -XX:-UseAdaptiveSizePolicy -Xmn128M".to_string(),
                        );
                    }
                }

                self.input = String::new();
                self.selected_version = 0;
                self.id = None;

                return Some(WindowType::Home);
            }
            KeyCode::Down => {
                if self.selected_version + 1 != self.versions.len() {
                    self.selected_version += 1;
                } else {
                    self.selected_version = 0;
                }
            }
            KeyCode::Up => {
                if self.selected_version > 0 {
                    self.selected_version -= 1;
                } else {
                    self.selected_version = self.versions.len() - 1;
                }
            }
            KeyCode::Backspace => {
                self.input.pop();
            }
            KeyCode::Char(ch) => {
                self.input.push(ch);
            }
            _ => {}
        }
        None
    }

    fn render<B>(&mut self, frame: &mut Frame<B>, _: Option<Rect>) where B: Backend {
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
            .constraints([Constraint::Ratio(1, 4), Constraint::Ratio(1, 2), Constraint::Ratio(1, 4)].as_ref())
            .split(layout[1]);

        let block = Block::default().borders(Borders::ALL).title("Profile creator");
        frame.render_widget(block, layout[1]);

        let paragraph = Paragraph::new(Spans::from(self.input.as_str()))
            .block(Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD))
                .title("Name"));

        frame.render_widget(paragraph, chunks[0]);

        let versions: Vec<ListItem> = self.versions
            .iter()
            .map(|v| ListItem::new(v.id.as_str()))
            .collect();
        let list = List::new(versions)
            .block(Block::default().borders(Borders::ALL).title("Versions"))
            .highlight_style(Style::default().fg(Color::LightGreen).add_modifier(Modifier::BOLD))
            .highlight_symbol(">");
        frame.render_stateful_widget(list, chunks[1], &mut self.list_state);
        self.list_state.select(Some(self.selected_version));

        let style = Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD);
        let paragraph = Paragraph::new(Spans::from(vec![" Press ".into(), Span::styled("enter", style), " to submit".into()]))
            .wrap(Wrap { trim: true })
            .block(Block::default().borders(Borders::TOP));

        frame.render_widget(paragraph, chunks[2]);
    }
}