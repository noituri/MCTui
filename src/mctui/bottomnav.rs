use tui::backend::Backend;
use tui::layout::{Rect, Layout, Direction, Constraint};
use tui::Frame;
use tui::widgets::{Borders, Block, Widget, SelectableList};
use tui::style::{Style, Color, Modifier};
use super::app::{WinWidget, Window};
use crate::SETTINGS;
use termion::event::Key;
use core::borrow::Borrow;
use std::thread;
use crossbeam_channel::Sender;

pub struct BottomNav {
    pub items: Items,
    pub selected: usize,
    pub sender: Option<Sender<String>>,
    profile_selector: bool
}

pub struct Items {
    middle: Vec<String>
}

impl WinWidget for BottomNav {
    fn new() -> BottomNav {
        BottomNav{
            items: Items {
              middle: vec!["Play".to_string(), "Selected Profile: ${profile}".to_string()],
            },
            selected: 0,
            sender: None,
            profile_selector: false
        }
    }

    fn handle_events(&mut self, key: Key) -> Option<Window> {
        match key {
            Key::Char('\n') => {
                if self.profile_selector {
                    if self.selected != 0 {
                        let mut settings = crate::SETTINGS.lock().unwrap();
                        settings.profiles.selected = settings.profiles.profiles[self.selected - 1].id.to_owned();

                        crate::universal::save_settings(&*settings);
                    }

                    self.selected = 0;
                    self.profile_selector = false;
                    self.items.middle = vec!["Play".to_string(), "Selected Profile: ${profile}".to_string()];
                    return None
                }

                match self.selected {
                    0 => {
                        let settings = crate::SETTINGS.lock().unwrap();
                        let selected = settings.profiles.selected.to_owned();
                        std::mem::drop(settings);

                        match self.sender.to_owned() {
                            Some(sender) => {
                                thread::spawn(move || {
                                    crate::utils::launch::prepare_game(&selected, sender.clone());
                                });
                            },
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
                        self.selected = 0;
                        self.items.middle = temp_vec;
                    }
                    _ => {}
                }
            }
            Key::Down => {
                if self.selected + 1 != self.items.middle.len() {
                    self.selected += 1;
                } else {
                    self.selected = 0;
                }
            }
            Key::Up => {
                if self.selected > 0 {
                    self.selected -= 1;
                } else {
                    self.selected = self.items.middle.len() - 1;
                }
            }
            _ => {}
        }

        None
    }

    fn render<B>(&mut self, backend: &mut Frame<B>, rect: Option<Rect>) where B: Backend {
        let style = Style::default().fg(Color::Black).bg(Color::White);

        let chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(33), Constraint::Percentage(33), Constraint::Percentage(33)].as_ref())
            .split(rect.unwrap());

        {
            if !self.profile_selector {
                let settings = SETTINGS.lock().unwrap();
                let selected_profile = settings.profiles.selected.to_owned();
                std::mem::drop(settings);

                let profile = crate::universal::get_profile(&selected_profile);
                match profile {
                    Some(p) => self.items.middle[1] = self.items.middle[1].replace("${profile}", &p.name),
                    None => self.items.middle[1] = "Select Profile".to_string()
                }
            }
        }

        Block::default().borders(Borders::ALL).title("(W.I.P) Mojang API").render(backend, chunks[0]);
        Block::default().borders(Borders::ALL).title("(W.I.P) Settings & Accounts").render(backend, chunks[2]);

        SelectableList::default()
            .block(Block::default().borders(Borders::ALL).title("Game"))
            .items(&self.items.middle)
            .select(Some(self.selected))
            .highlight_style(style.fg(Color::LightGreen).modifier(Modifier::BOLD))
            .highlight_symbol(">")
            .render(backend, chunks[1]);
    }
}