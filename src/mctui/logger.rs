use tui::widgets::{Block, Borders, Paragraph, Text, Widget};
use std::slice::Iter;
use tui::Frame;
use tui::layout::{Rect, Layout, Constraint};
use tui::backend::Backend;

pub fn render_logger<B>(backend: & mut Frame<B>, rect: Rect, logs: Vec<Text>) where B: Backend {
    let mut para = Paragraph::new( logs.iter());
    para.block(Block::default()
        .title("Logs")
        .borders(Borders::ALL)).render(backend, rect);
}