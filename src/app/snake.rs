use ratatui::prelude::Color;
use ratatui::widgets::canvas::{Painter, Shape};
use crate::app::board::Direction;

#[derive(Debug)]
pub struct Snake {
    points: Vec<(f64, f64)>
}

impl Default for Snake {
    fn default() -> Self {
        Self {
            points: vec![(2.0, 10.0)]
        }
    }
}

impl Shape for Snake {
    fn draw(&self, painter: &mut Painter) {
        for (x, y) in self.points.iter() {
            if let Some((x, y)) = painter.get_point(*x, *y) {
                painter.paint(x, y, Color::Green);
            }
        }
    }
}

impl Snake {
    pub fn head(&self) -> Option<&(f64, f64)> {
        self.points.get(0)
    }
    pub fn move_snake(&mut self, direction: Direction) {
        let popped = self.points.pop();
        if let Some(popped) = popped {
            let delta = match direction {
                Direction::Up => (0.0, 1.0),
                Direction::Down => (0.0, -1.0),
                Direction::Left => (-1.0, 0.0),
                Direction::Right => (1.0, 0.0),
            };
            self.points.push((popped.0 + delta.0, popped.1 + delta.1));
        }
    }
}
