use crate::mctui::app::App;
use termion::event::Key;
use termion::input::MouseTerminal;
use termion::raw::IntoRawMode;
use termion::screen::AlternateScreen;
use tui::backend::TermionBackend;
use crate::mctui::events::{Events, Event};
use tui::Terminal;
use tui::widgets::{Block, Widget, Borders};

pub fn start_tui() -> Result<(), failure::Error>  {
    let stdout = std::io::stdout().into_raw_mode()?;
    let stdout = MouseTerminal::from(stdout);
    let stdout = AlternateScreen::from(stdout);
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    terminal.hide_cursor()?;

    let mut app = App::new();
    let events = Events::new();

    loop {
        terminal.draw(|mut f| {
            let size = f.size();

            Block::default().borders(Borders::ALL).render(&mut f, size);
        })?;

        if handle_events(&events).is_none() {
            break;
        }
    }

    Ok(())
}

fn handle_events(events: &Events) -> Option<()> {
    match events.next().unwrap() {
        Event::Input(input) => {
            match input {
                Key::Char('q') => return None,
                _ => {}
            }
        },
        _ => {}
    }
    Some(())
}