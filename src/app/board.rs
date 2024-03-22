use ratatui::prelude::*;
use ratatui::symbols::border;
use ratatui::widgets::{Block, Borders};
use ratatui::widgets::block::Title;


#[derive(Debug, Default)]
pub struct Board {}

impl Widget for &Board {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let title = Title::from("hello player");
        Block::default()
            .title(title.alignment(Alignment::Center))
            .borders(Borders::ALL)
            .border_set(border::THICK)
            .render(area, buf);
    }
}
