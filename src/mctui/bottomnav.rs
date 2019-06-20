use tui::backend::Backend;
use tui::layout::Rect;
use tui::Frame;
use tui::widgets::{Borders, Block, Widget, SelectableList};
use tui::style::{Style, Color, Modifier};

pub struct BottomNav<'a> {
    pub items: Items<'a>,
}

pub struct Items<'a> {
    middle: Vec<&'a str>
}

impl<'a> BottomNav<'a> {
    pub fn new() -> BottomNav<'a> {
        BottomNav{
            items: Items {
              middle: vec!["Play"],
            }
        }
    }

    pub fn render<B>(&self, backend: &mut Frame<B>, rect: Rect) where B: Backend {
        let style = Style::default().fg(Color::Black).bg(Color::White);

        SelectableList::default()
            .block(Block::default().borders(Borders::ALL).title("Options"))
            .items(&self.items.middle)
            .select(Some(0))
            .highlight_style(style.fg(Color::LightGreen).modifier(Modifier::BOLD))
            .highlight_symbol(">")
            .render(backend, rect);
    }
}