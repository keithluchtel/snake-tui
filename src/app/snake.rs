use ratatui::widgets::canvas::{Painter, Shape};

#[derive(Debug, Default)]
pub struct Snake {}

impl Shape for Snake {
    fn draw(&self, _: &mut Painter) {

    }
}
