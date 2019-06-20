use tui::widgets::{Block, Borders, Text, Widget, List};
use tui::Frame;
use tui::layout::Rect;
use tui::backend::Backend;
use crossbeam_channel::Receiver;
use super::app::WinWidget;

pub struct LoggerFrame {
    pub receiver: Option<Receiver<String>>,
    output: Vec<String>
}

impl WinWidget for LoggerFrame {
    fn new() -> LoggerFrame {
        LoggerFrame {
            receiver: None,
            output: Vec::new()
        }
    }

    fn render<B>(&mut self, backend: &mut Frame<B>, rect: Option<Rect>) where B: Backend {
        let receiver = self.receiver.to_owned().unwrap();
        let rect = rect.unwrap();

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