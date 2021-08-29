use crate::mctui::app::App;
use crate::mctui::events::{Event, Events};
use crate::SettingsPtr;
use crossbeam_channel::unbounded;
use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::error::Error;
use tui::{backend::CrosstermBackend, Terminal};

pub async fn start_tui(settings: SettingsPtr) -> Result<(), Box<dyn Error>> {
    enable_raw_mode()?;
    let mut stdout = std::io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;

    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    terminal.hide_cursor()?;
    terminal.clear()?;

    let mut app = App::new(settings.clone()).await;

    let (s, r) = unbounded();
    app.windows.home.bottom_nav.sender = Some(s);
    app.windows.home.logger.receiver = Some(r);
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
    if let Event::Input(input) = events.next().unwrap() {
        if input == KeyCode::Char('q') {
            return None;
        }

        app.handle_events(input).await;
    }

    Some(())
}
