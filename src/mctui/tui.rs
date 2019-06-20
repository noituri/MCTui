use crate::mctui::app::{App, Window};
use crate::SETTINGS;
use crate::universal::save_settings;
use termion::event::Key;
use termion::input::MouseTerminal;
use termion::raw::IntoRawMode;
use termion::screen::AlternateScreen;
use tui::backend::TermionBackend;
use crate::mctui::events::{Events, Event, Config};
use tui::Terminal;
use tui::widgets::{Block, Widget, Borders, SelectableList};
use tui::layout::{Layout, Direction, Constraint};
use crate::mctui::logger::LoggerFrame;
use crate::mctui::welcome::{WelcomeWindow, Selected};
use std::thread;
use tui::style::{Color, Modifier, Style};
use std::sync::{Arc, Mutex};
use crossbeam_channel::{unbounded, Sender, Receiver};

pub fn start_tui() -> Result<(), failure::Error> {
    let stdout = std::io::stdout().into_raw_mode()?;
    let stdout = MouseTerminal::from(stdout);
    let stdout = AlternateScreen::from(stdout);
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    terminal.hide_cursor()?;

    let mut app = App::new();
    let events = Events::new();

    let (s, r) = unbounded();

//    let mut welcome_window = WelcomeWindow::new();
//    let mut logger_frame = LoggerFrame::new();

    loop {
        terminal.draw(|mut f| {
            let size = f.size();

            match app.current_window {
                Window::Home => {
                    let chunks = Layout::default()
                        .direction(Direction::Vertical)
                        .constraints([Constraint::Percentage(80), Constraint::Percentage(20)].as_ref())
                        .split(size);
                    app.windows.home.render(&mut f, chunks[0], r.clone());

                    let style = Style::default().fg(Color::Black).bg(Color::White);
                    SelectableList::default()
                        .block(Block::default().borders(Borders::ALL).title("Options"))
                        .items(&vec!("Play"))
                        .select(Some(0))
                        .highlight_style(style.fg(Color::LightGreen).modifier(Modifier::BOLD))
                        .highlight_symbol(">")
                        .render(&mut f, chunks[1]);
                },
                Window::Welcome => {
                    let chunks = Layout::default()
                        .direction(Direction::Vertical)
                        .margin(2)
                        .constraints([Constraint::Length(3), Constraint::Max(12), Constraint::Max(1)].as_ref())
                        .split(
                            Layout::default().direction(Direction::Horizontal)
                                .constraints([
                                    Constraint::Percentage(30),
                                    Constraint::Percentage(40),
                                    Constraint::Percentage(30)
                                ].as_ref()).split(size)[1]);

                    app.windows.welcome.render(&mut f, chunks[1]);
                }
            }
        })?;

        if handle_events(&events, s.clone(), &mut app).is_none() {
            break;
        }
    }

    Ok(())
}

fn handle_events(events: &Events, sender: Sender<String>, app: &mut App) -> Option<()> {
    match events.next().unwrap() {
        Event::Input(input) => {
            match input {
                Key::Char('q') => return None,
                Key::Char('\n') => {
                    match app.current_window {
                        Window::Welcome => {
                            let mut settings = SETTINGS.lock().unwrap();
                            settings.auth.username = app.windows.welcome.input.0.to_owned();
                            save_settings(&*settings);
                        }
                        Window::Home => {
                            let settings = crate::SETTINGS.lock().unwrap();
                            let selected = settings.profiles.selected.to_owned();
                            std::mem::drop(settings);

                            thread::spawn(move || {
                                crate::utils::launch::prepare_game(&selected, sender)
                            });
                        }
                    }
                }
                Key::Down | Key::Up | Key::Char('\t') => {
                    match app.windows.welcome.selected {
                        Selected::Username => app.windows.welcome.selected = Selected::Password,
                        Selected::Password => app.windows.welcome.selected = Selected::Username
                    }
                }
                Key::Backspace => {
                    match app.current_window {
                        Window::Welcome => {
                            match app.windows.welcome.selected {
                                Selected::Username => app.windows.welcome.input.0.pop(),
                                Selected::Password => app.windows.welcome.input.1.pop()
                            };
                        }
                        _ => {}
                    }
                }
                Key::Char(ch) => {
                    match app.current_window {
                        Window::Welcome => {
                            match app.windows.welcome.selected {
                                Selected::Username => app.windows.welcome.input.0.push(ch),
                                Selected::Password => app.windows.welcome.input.1.push(ch)
                            }
                        }
                        _ => {}
                    }
                },
                _ => {}
            }
        }
        _ => {}
    }
    Some(())
}