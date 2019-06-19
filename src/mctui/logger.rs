use tui::widgets::{Block, Borders, Paragraph, Text, Widget, List};
use std::slice::Iter;
use tui::Frame;
use tui::layout::{Rect, Layout, Constraint};
use tui::backend::Backend;
use std::sync::{Arc, Mutex};
use std::thread;
use crossbeam_channel::Receiver;
use lazy_static::lazy_static;

lazy_static! {
    static ref OUTPUT: Mutex<Vec<String>> = Mutex::new(Vec::new());
}

pub fn render_logger<B>(backend: &mut Frame<B>, rect: Rect, receiver: Receiver<String>) where B: Backend {
    let mut output = OUTPUT.lock().unwrap();
    for log in receiver.try_iter() {
        output.push(log);
    }

    if output.len() as u16 >= rect.height - 2 {
        output.remove(0);
    }

    let logs = output.iter().map(|log| Text::raw(log));

    List::new(logs)
        .block(Block::default()
            .borders(Borders::ALL)
            .title("Logs"))
        .render(backend, rect);
}