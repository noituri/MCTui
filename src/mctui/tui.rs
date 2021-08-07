use crate::mctui::app::{App, TuiWidget, WindowType};
use crate::mctui::events::{Event, Events};
use crossbeam_channel::unbounded;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::{error::Error, io::Write};
use tui::{backend::CrosstermBackend, Terminal};

pub async fn start_tui() -> Result<(), Box<dyn Error>> {
    enable_raw_mode()?;
    let mut stdout = std::io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;

    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    terminal.hide_cursor()?;
    terminal.clear()?;

    let mut app = App::new().await;

    let (s, r) = unbounded();
    app.windows.home.sender = Some(s);
    app.windows.home.receiver = Some(r);
    let events = Events::new();
    loop {
        terminal.draw(|mut f| {
            app.render(&mut f);
        })?;

        if handle_events(&events, &mut app).await.is_none() {
            disable_raw_mode()?;
            execute!(
                terminal.backend_mut(),
                LeaveAlternateScreen,
                DisableMouseCapture
            )?;
            break;
        }
    }

    Ok(())
}

async fn handle_events(events: &Events, app: &mut App) -> Option<()> {
    match events.next().unwrap() {
        Event::Input(input) => {
            if input == KeyCode::Char('q') {
                return None;
            }

            match &app.current_window {
                // Window::Home(_) => {
                //     match app.windows.home.handle_events(input) {
                //         Some(route) => app.current_window = route,
                //         None => {}
                //     }

                //     if app.windows.home.tab_index == 1 {
                //         match app.windows.home.profiles_tab.handle_events(input) {
                //             Some(route) => {
                //                 match &route {
                //                     Window::ProfileCreator(id) => {
                //                         if id != "" {
                //                             app.windows.profile_creator.id = Some(id.to_owned());
                //                             match crate::universal::get_profile(&id) {
                //                                 Some(profile) => {
                //                                     app.windows.profile_creator.input = profile.name.to_owned();

                //                                     for (i, v) in app.windows.profile_creator.versions.iter().enumerate() {
                //                                         if v.id == profile.version {
                //                                             app.windows.profile_creator.selected_version = i;
                //                                             break;
                //                                         }
                //                                     }
                //                                 }
                //                                 None => {}
                //                             }
                //                         }
                //                     }
                //                     _ => {}
                //                 }

                //                 app.current_window = route
                //             },
                //             None => {}
                //         }
                //     } else {
                //         match app.windows.home.bottom_nav.handle_events(input) {
                //             Some(route) => app.current_window = route,
                //             None => {}
                //         }
                //     }
                // },
                _ => app.handle_events(input).await,
            }
        }
        _ => {}
    }

    Some(())
}
