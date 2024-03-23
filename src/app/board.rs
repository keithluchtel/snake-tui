use ratatui::widgets::canvas::{Painter, Shape};
use crate::app::snake::Snake;


#[derive(Debug, Default)]
pub struct Board {
    pub x_bounds: [i32; 2],
    pub y_bounds: [i32; 2],
    snake: Snake,
}

impl Shape for Board {
    fn draw(&self, painter: &mut Painter) {
        self.snake.draw(painter);
    }
}

impl Board {
    pub fn process_tick(&mut self) {
        self.snake.move_snake();
    }
}
