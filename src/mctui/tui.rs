use crate::mctui::app::{App, Window, WinWidget};
use crate::structs::libraries::Libraries;
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
    app.windows.home.sender = Some(s);
    app.windows.home.receiver = Some(r);

    loop {
        terminal.draw(|mut f| {
            match app.current_window {
                Window::Home => app.windows.home.render(&mut f, None),
                Window::Welcome => app.windows.welcome.render(&mut f, None),
                Window::ProfileCreator => app.windows.profile_creator.render(&mut f, None),
            }
        })?;

        if handle_events(&events, &mut app).is_none() {
            break;
        }
    }

    Ok(())
}

fn handle_events(events: &Events, app: &mut App) -> Option<()> {
    match events.next().unwrap() {
        Event::Input(input) => {
            if input == Key::Char('q') {
                return None
            }

            match app.current_window {
                Window::Welcome => {
                    match app.windows.welcome.handle_events(input) {
                        Some(route) => app.current_window = route,
                        None => {}
                    }

                },
                Window::ProfileCreator => {
                    match app.windows.profile_creator.handle_events(input) {
                        Some(route) => app.current_window = route,
                        None => {}
                    }

                },
                Window::Home => {
                    match app.windows.home.handle_events(input) {
                        Some(route) => app.current_window = route,
                        None => {}
                    }
                    match app.windows.home.profiles_tab.handle_events(input) {
                        Some(route) => app.current_window = route,
                        None => {}
                    }
                },
            }
        }
        _ => {}
    }

    Some(())
}