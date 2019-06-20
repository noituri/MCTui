use crate::mctui::app::{App, Window, WinWidget};
use crate::SETTINGS;
use crate::universal::save_settings;
use termion::event::Key;
use termion::input::MouseTerminal;
use termion::raw::IntoRawMode;
use termion::screen::AlternateScreen;
use tui::backend::TermionBackend;
use crate::mctui::events::{Events, Event};
use tui::Terminal;
use crate::mctui::welcome::Selected;
use std::thread;
use crossbeam_channel::{unbounded, Sender};

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
    app.windows.home.receiver = Some(r);

    loop {
        terminal.draw(|mut f| {
            match app.current_window {
                Window::Home => app.windows.home.render(&mut f, None),
                Window::Welcome => app.windows.welcome.render(&mut f, None)
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