use crate::mctui::app::App;
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
use crate::mctui::welcome::WelcomeWindow;
use std::thread;
use tui::style::{Color, Modifier, Style};
use std::sync::{Arc, Mutex};
use crossbeam_channel::{unbounded, Sender, Receiver};

pub fn start_tui() -> Result<(), failure::Error>  {
    let stdout = std::io::stdout().into_raw_mode()?;
    let stdout = MouseTerminal::from(stdout);
    let stdout = AlternateScreen::from(stdout);
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    terminal.hide_cursor()?;

    let mut app = App::new();
    let events = Events::new();

    let (s, r) = unbounded();

    let mut welcome_window = WelcomeWindow::new();
    let mut logger_frame = LoggerFrame::new();

    loop {
        terminal.draw(|mut f| {
            let size = f.size();

//            let chunks = Layout::default()
//                .direction(Direction::Vertical)
//                .constraints([Constraint::Percentage(80), Constraint::Percentage(20)].as_ref())
//                .split(size);

//            logger_frame.render(&mut f, chunks[0], r.clone());

//            let style = Style::default().fg(Color::Black).bg(Color::White);
//            SelectableList::default()
//                .block(Block::default().borders(Borders::ALL).title("Options"))
//                .items(&vec!("Play"))
//                .select(Some(0))
//                .highlight_style(style.fg(Color::LightGreen).modifier(Modifier::BOLD))
//                .highlight_symbol(">")
//                .render(&mut f, chunks[1]);

            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .margin(18)
                .constraints([Constraint::Percentage(100)].as_ref())
                .split(size);

            welcome_window.render(&mut f, chunks[0]);
        })?;

        if handle_events(&events, s.clone(), &mut welcome_window).is_none() {
            break;
        }
    }

    Ok(())
}

fn handle_events(events: &Events, sender: Sender<String>, w: &mut WelcomeWindow) -> Option<()> {
    match events.next().unwrap() {
        Event::Input(input) => {
            match input {
                Key::Char('q') => return None,
                Key::Char('\n') => {
                    let settings = crate::SETTINGS.lock().unwrap();
                    let selected = settings.profiles.selected.to_owned();
                    std::mem::drop(settings);

                    thread::spawn(move || {
                        crate::utils::launch::prepare_game(&selected, sender)
                    });

                },
                Key::Char(ch) => w.input.0.push(ch),
                _ => {}
            }
        },
        _ => {}
    }
    Some(())
}