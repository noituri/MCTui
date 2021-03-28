use crate::mctui::app::{App, WindowType, TuiWidget};
use crate::mctui::events::{Events, Event};
use tui::{Terminal, backend::CrosstermBackend};
use crossbeam_channel::unbounded;
use crossterm::{
    execute,
    terminal::{enable_raw_mode, disable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    event::{self, DisableMouseCapture, EnableMouseCapture, KeyCode}
};
use std::io::Write;

pub fn start_tui() -> Result<(), failure::Error> {
    enable_raw_mode()?;
    let mut stdout = std::io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;

    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    terminal.hide_cursor()?;

    terminal.clear()?;
    let mut app = App::new();
    let events = Events::new();

    // let (s, r) = unbounded();
    // app.windows.home.sender = Some(s);
    // app.windows.home.receiver = Some(r);

    loop {
        terminal.draw(|mut f| {
            app.render(&mut f);
            // match app.current_window {
            //     Window::Home(_) => app.windows.home.render(&mut f, None),
            //     Window::Welcome(_) => app.windows.welcome.render(&mut f, None),
            //     Window::ProfileCreator(_) => app.windows.profile_creator.render(&mut f, None),
            // }
        })?;

        // if handle_events(&events, &mut app).is_none() {
        //     break;
        // }
    }

    Ok(())
}

fn handle_events(events: &Events, app: &mut App) -> Option<()> {
    // match events.next().unwrap() {
    //     Event::Input(input) => {
    //         if input == Key::Char('q') {
    //             return None
    //         }

    //         match &app.current_window {
    //             Window::Welcome(_) => {
    //                 match app.windows.welcome.handle_events(input) {
    //                     Some(route) => app.current_window = route,
    //                     None => {}
    //                 }

    //             },
    //             Window::ProfileCreator(_) => {
    //                 match app.windows.profile_creator.handle_events(input) {
    //                     Some(route) => app.current_window = route,
    //                     None => {}
    //                 }

    //             },
    //             Window::Home(_) => {
    //                 match app.windows.home.handle_events(input) {
    //                     Some(route) => app.current_window = route,
    //                     None => {}
    //                 }

    //                 if app.windows.home.tab_index == 1 {
    //                     match app.windows.home.profiles_tab.handle_events(input) {
    //                         Some(route) => {
    //                             match &route {
    //                                 Window::ProfileCreator(id) => {
    //                                     if id != "" {
    //                                         app.windows.profile_creator.id = Some(id.to_owned());
    //                                         match crate::universal::get_profile(&id) {
    //                                             Some(profile) => {
    //                                                 app.windows.profile_creator.input = profile.name.to_owned();

    //                                                 for (i, v) in app.windows.profile_creator.versions.iter().enumerate() {
    //                                                     if v.id == profile.version {
    //                                                         app.windows.profile_creator.selected_version = i;
    //                                                         break;
    //                                                     }
    //                                                 }
    //                                             }
    //                                             None => {}
    //                                         }
    //                                     }
    //                                 }
    //                                 _ => {}
    //                             }

    //                             app.current_window = route
    //                         },
    //                         None => {}
    //                     }
    //                 } else {
    //                     match app.windows.home.bottom_nav.handle_events(input) {
    //                         Some(route) => app.current_window = route,
    //                         None => {}
    //                     }
    //                 }
    //             },
    //         }
    //     }
    //     _ => {}
    // }

    Some(())
}