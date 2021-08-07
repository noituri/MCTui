use super::app::{TuiWidget, WindowType};
use crate::constants::VERSIONS;
use crate::structs::libraries::Libraries;
use crate::structs::versions;
use crossterm::event::KeyCode;
use tui::backend::Backend;
use tui::layout::{Constraint, Direction, Layout, Rect};
use tui::style::{Color, Modifier, Style};
use tui::widgets::{Block, Borders, Paragraph};
use tui::{
    text::{Span, Spans},
    widgets::{List, ListItem, ListState, Wrap},
    Frame,
};
use async_trait::async_trait;

pub struct ProfileCreatorWindow {
    pub input: String,
    pub id: Option<String>,
    pub versions: Vec<versions::Version>,
    list_state: ListState,
}

impl ProfileCreatorWindow {
    pub async fn new() -> Self {
        let versions_resp: versions::Versions = reqwest::get(VERSIONS).await.unwrap().json().await.unwrap();
        let mut list_state = ListState::default();
        list_state.select(Some(0));

        Self {
            list_state,
            input: String::new(),
            id: None,
            versions: versions_resp.versions,
        }
    }
}

#[async_trait]
impl TuiWidget for ProfileCreatorWindow {
    async fn handle_events(&mut self, key: KeyCode) -> Option<WindowType> {
        let selected_item = self.list_state.selected().unwrap_or_default();
        match key {
            KeyCode::Enter => {
                let selected_version = &self.versions[selected_item];
                //TODO check connection
                let assets_resp: Libraries = reqwest::get(selected_version.url.as_str())
                    .await
                    .unwrap()
                    .json()
                    .await
                    .unwrap();

                match self.id.to_owned() {
                    Some(id) => {
                        crate::universal::edit_profile(
                            id,
                            self.input.to_owned(),
                            selected_version.id.to_owned(),
                        );
                    }
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
                self.list_state.select(Some(0));
                self.id = None;

                return Some(WindowType::Home);
            }
            KeyCode::Down => {
                if selected_item + 1 != self.versions.len() {
                    self.list_state.select(Some(selected_item + 1));
                } else {
                    self.list_state.select(Some(0));
                }
            }
            KeyCode::Up => {
                if selected_item > 0 {
                    self.list_state.select(Some(selected_item - 1));
                } else {
                    self.list_state.select(Some(self.versions.len() - 1));
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
            .constraints(
                [
                    Constraint::Ratio(1, 4),
                    Constraint::Ratio(1, 2),
                    Constraint::Ratio(1, 4),
                ]
                .as_ref(),
            )
            .split(layout[1]);

        let block = Block::default()
            .borders(Borders::ALL)
            .title("Profile creator");
        frame.render_widget(block, layout[1]);

        let paragraph = Paragraph::new(Spans::from(self.input.as_str())).block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(
                    Style::default()
                        .fg(Color::Cyan)
                        .add_modifier(Modifier::BOLD),
                )
                .title("Name"),
        );

        frame.render_widget(paragraph, chunks[0]);

        let versions: Vec<ListItem> = self
            .versions
            .iter()
            .map(|v| ListItem::new(v.id.as_str()))
            .collect();
        let list = List::new(versions)
            .block(Block::default().borders(Borders::ALL).title("Versions"))
            .highlight_style(
                Style::default()
                    .fg(Color::LightGreen)
                    .add_modifier(Modifier::BOLD),
            )
            .highlight_symbol(">");
        frame.render_stateful_widget(list, chunks[1], &mut self.list_state);

        let style = Style::default()
            .fg(Color::Cyan)
            .add_modifier(Modifier::BOLD);
        let paragraph = Paragraph::new(Spans::from(vec![
            " Press ".into(),
            Span::styled("enter", style),
            " to submit".into(),
        ]))
        .wrap(Wrap { trim: true })
        .block(Block::default().borders(Borders::TOP));

        frame.render_widget(paragraph, chunks[2]);
    }
}
