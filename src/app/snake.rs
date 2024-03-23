use ratatui::prelude::Color;
use ratatui::widgets::canvas::{Painter, Shape};

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
    pub fn move_snake(&mut self) {
        let popped = self.points.pop();
        if let Some(popped) = popped {
            self.points.push((popped.0 + 1.0, popped.1));
        }
    }
}
