use crossbeam_channel::Receiver;
use crossterm::event::KeyCode;
use tui::backend::Backend;
use tui::layout::Rect;
use tui::widgets::{Block, Borders, List, ListItem, Widget};
use tui::Frame;

use super::app::{TuiWidget, WindowType};

pub struct LoggerFrame {
    pub receiver: Option<Receiver<String>>,
    output: Vec<String>,
}

impl LoggerFrame {
    pub fn new() -> Self {
        Self {
            receiver: None,
            output: Vec::new(),
        }
    }
}

impl TuiWidget for LoggerFrame {
    fn handle_events(&mut self, _: KeyCode) -> Option<WindowType> {
        unimplemented!()
    }

    fn render<B>(&mut self, frame: &mut Frame<B>, rect: Option<Rect>)
    where
        B: Backend,
    {
        let receiver = self.receiver.to_owned().unwrap();
        let rect = rect.unwrap();

        for log in receiver.try_iter() {
            self.output.push(log);
        }

        if self.output.len() as u16 >= rect.height - 2 {
            self.output.remove(0);
        }

        let logs: Vec<ListItem> = self
            .output
            .iter()
            .map(|log| ListItem::new(log.as_str()))
            .collect();
        let list = List::new(logs).block(Block::default().borders(Borders::ALL).title("Logs"));
        frame.render_widget(list, rect);
    }
}
