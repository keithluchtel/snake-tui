use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::prelude::Widget;

#[derive(Debug, Default)]
pub struct Snake {}

impl Widget for Snake {
    fn render(self, area: Rect, buf: &mut Buffer) {
    }
}
