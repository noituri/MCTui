use tui::backend::Backend;
use tui::layout::{Layout, Direction, Constraint, Rect};
use tui::Frame;
use super::logger::LoggerFrame;
use super::bottomnav::BottomNav;
use super::profilestab::ProfilesTab;
use super::app::{WinWidget, Window};
use crossbeam_channel::{Receiver, Sender};
use termion::event::Key;
use std::thread;
use tui::widgets::{Tabs, Block, Widget, Borders};
use tui::style::{Style, Color};

pub struct HomeWindow<'a> {
    pub sender: Option<Sender<String>>,
    pub receiver: Option<Receiver<String>>,
    pub tab_index: usize,
    logger: LoggerFrame,
    bottom_nav: BottomNav<'a>,
    pub profiles_tab: ProfilesTab
}

impl<'a> WinWidget for HomeWindow<'a> {
    fn new() -> HomeWindow<'a> {
        HomeWindow {
            sender: None,
            receiver: None,
            tab_index: 0,
            logger: LoggerFrame::new(),
            bottom_nav: BottomNav::new(),
            profiles_tab: ProfilesTab::new()
        }
    }

    fn handle_events(&mut self, key: Key) -> Option<Window> {
        match key {
            Key::Char('\n') => {
                match self.tab_index {
                    0 => {
                        let settings = crate::SETTINGS.lock().unwrap();
                        let selected = settings.profiles.selected.to_owned();
                        std::mem::drop(settings);

                        match self.sender.to_owned() {
                            Some(sender) => {
                                thread::spawn(move || {
                                    crate::utils::launch::prepare_game(&selected, sender);
                                });
                            },
                            None => {}
                        }
                    }
                    _ => {}
                }
            }
            Key::Char('\t') => {
                self.tab_index = match self.tab_index {
                    0 => 1,
                    1 => 0,
                    _ => 0
                }
            }
            _ => {}
        }

        None
    }

    fn render<B>(&mut self, backend: &mut Frame<B>, _rect: Option<Rect>) where B: Backend {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Percentage(8), Constraint::Percentage(72), Constraint::Percentage(20)].as_ref())
            .split(backend.size());

        Tabs::default()
            .block(Block::default().borders(Borders::ALL).title("Tabs"))
            .titles(&vec!["Home", "Profiles"])
            .select(self.tab_index)
            .style(Style::default().fg(Color::Cyan))
            .highlight_style(Style::default().fg(Color::Yellow))
            .render(backend, chunks[0]);

        match self.tab_index {
            0 => {
                self.logger.receiver = self.receiver.to_owned();
                self.logger.render(backend, Some(chunks[1]));
                self.bottom_nav.render(backend, Some(chunks[2]));
            }
            1 => {
                self.profiles_tab.render(backend, None);
            }
            _ => {}
        }
    }
}