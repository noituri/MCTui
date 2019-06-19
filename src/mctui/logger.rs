use tui::widgets::{Block, Borders, Paragraph, Text, Widget, List};
use std::slice::Iter;
use tui::Frame;
use tui::layout::{Rect, Layout, Constraint};
use tui::backend::Backend;
use std::sync::{Arc, Mutex};
use std::thread;
use crossbeam_channel::Receiver;

//TODO trait

pub struct LoggerFrame {
    output: Vec<String>
}

impl LoggerFrame {
    pub fn new() -> LoggerFrame {
        LoggerFrame {
            output: Vec::new()
        }
    }

    pub fn render<B>(&mut self, backend: &mut Frame<B>, rect: Rect, receiver: Receiver<String>) where B: Backend {
        for log in receiver.try_iter() {
            self.output.push(log);
        }

        if self.output.len() as u16 >= rect.height - 2 {
            self.output.remove(0);
        }

        let logs = self.output.iter().map(|log| Text::raw(log));

        List::new(logs)
            .block(Block::default()
                .borders(Borders::ALL)
                .title("Logs"))
            .render(backend, rect);
    }
}