use tui::widgets::{Block, Borders, Paragraph, Text, Widget};
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

    let logs: Vec<Text> = output.iter().map(|log| Text::raw(log)).collect();
    let mut para = Paragraph::new(logs.iter());
    para.block(Block::default()
        .title("Logs")
        .borders(Borders::ALL)).render(backend, rect);
}