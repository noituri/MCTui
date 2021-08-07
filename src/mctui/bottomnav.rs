use crate::SETTINGS;
use async_trait::async_trait;
use crossbeam_channel::Sender;
use crossterm::event::KeyCode;
use tui::layout::{Constraint, Direction, Layout, Rect};
use tui::style::{Color, Modifier, Style};
use tui::Frame;
use tui::{
    backend::Backend,
    widgets::{Block, Borders, List, ListItem, ListState},
};

use super::app::{TuiWidget, WindowType};

pub struct BottomNav {
    pub items: Items,
    pub sender: Option<Sender<String>>,
    profile_selector: bool,
}

pub struct Items {
    state: ListState,
    middle: Vec<String>,
}

impl BottomNav {
    pub fn new() -> Self {
        let mut state = ListState::default();
        state.select(Some(0));
        Self {
            items: Items {
                state,
                middle: vec![
                    "Play".to_string(),
                    "Selected Profile: ${profile}".to_string(),
                ],
            },
            sender: None,
            profile_selector: false,
        }
    }
}

#[async_trait]
impl TuiWidget for BottomNav {
    async fn handle_events(&mut self, key: KeyCode) -> Option<WindowType> {
        let selected_item = self.items.state.selected().unwrap_or_default();
        match key {
            KeyCode::Enter => {
                if self.profile_selector {
                    if selected_item != 0 {
                        let mut settings = crate::SETTINGS.lock().unwrap();
                        settings.profiles.selected =
                            settings.profiles.profiles[selected_item - 1].id.to_owned();

                        settings.save();
                    }

                    self.items.state.select(Some(0));
                    self.profile_selector = false;
                    self.items.middle = vec![
                        "Play".to_string(),
                        "Selected Profile: ${profile}".to_string(),
                    ];
                    return None;
                }

                match selected_item {
                    0 => {
                        let selected = {
                            let settings = crate::SETTINGS.lock().unwrap();
                            settings.profiles.selected.to_owned()
                        };

                        match self.sender.to_owned() {
                            Some(sender) => {
                                tokio::spawn(async move {
                                    crate::utils::launch::prepare_game(&selected, sender.clone())
                                        .await;
                                });
                            }
                            None => {}
                        }
                    }
                    1 => {
                        let settings = SETTINGS.lock().unwrap();
                        let mut temp_vec = vec!["<--".to_string()];
                        for p in settings.profiles.profiles.iter() {
                            temp_vec.push(p.name.to_owned());
                        }

                        self.profile_selector = true;
                        self.items.state.select(Some(0));
                        self.items.middle = temp_vec;
                    }
                    _ => {}
                }
            }
            KeyCode::Down => {
                if selected_item + 1 != self.items.middle.len() {
                    self.items.state.select(Some(selected_item + 1));
                } else {
                    self.items.state.select(Some(0));
                }
            }
            KeyCode::Up => {
                if selected_item > 0 {
                    self.items.state.select(Some(selected_item - 1));
                } else {
                    self.items.state.select(Some(self.items.middle.len() - 1));
                }
            }
            _ => {}
        }

        None
    }

    fn render<B>(&mut self, frame: &mut Frame<B>, rect: Option<Rect>)
    where
        B: Backend,
    {
        let style = Style::default().fg(Color::Black).bg(Color::White);

        let chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(
                [
                    Constraint::Percentage(33),
                    Constraint::Percentage(33),
                    Constraint::Percentage(33),
                ]
                .as_ref(),
            )
            .split(rect.unwrap());

        {
            if !self.profile_selector {
                let settings = SETTINGS.lock().unwrap();
                let selected_profile = settings.profiles.selected.to_owned();
                std::mem::drop(settings);

                let profile = crate::universal::get_profile(&selected_profile);
                match profile {
                    Some(p) => {
                        self.items.middle[1] = self.items.middle[1].replace("${profile}", &p.name)
                    }
                    None => self.items.middle[1] = "Select Profile".to_string(),
                }
            }
        }

        let block = Block::default()
            .borders(Borders::ALL)
            .title("(W.I.P) Mojang API");
        frame.render_widget(block, chunks[0]);
        let block = Block::default()
            .borders(Borders::ALL)
            .title("(W.I.P) Settings & Accounts");
        frame.render_widget(block, chunks[2]);

        let items: Vec<ListItem> = self
            .items
            .middle
            .iter()
            .map(|i| ListItem::new(i.as_str()))
            .collect();
        let list = List::new(items)
            .block(Block::default().borders(Borders::ALL).title("Game"))
            .highlight_style(style.fg(Color::LightGreen).add_modifier(Modifier::BOLD))
            .highlight_symbol(">");
        frame.render_stateful_widget(list, chunks[1], &mut self.items.state);
    }
}
