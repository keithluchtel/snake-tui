use ratatui::widgets::canvas::{Painter, Shape};
use crate::app::snake::Snake;


#[derive(Clone, Copy, Debug, Default)]
pub enum Direction {
    #[default]
    Up,
    Down,
    Left,
    Right
}

#[derive(Debug, Default)]
pub struct Board {
    pub x_bounds: [i32; 2],
    pub y_bounds: [i32; 2],
    snake: Snake,
    direction: Direction
}

impl Shape for Board {
    fn draw(&self, painter: &mut Painter) {
        self.snake.draw(painter);
    }
}

impl Board {
    pub fn process_tick(&mut self) {
        self.snake.move_snake(self.direction);
    }
    
    pub fn set_direction(&mut self, direction: Direction) {
        self.direction = direction;
    }
    
    pub fn check_collisions(&self) -> bool {
        let snake_head = self.snake.head();
        if let Some(head) = snake_head {
            match head {
                (x, _) if *x <= f64::from(self.x_bounds[0]) => return true,
                (x, _) if *x >= f64::from(self.x_bounds[1]) => return true,
                (x, _) if *x >= f64::from(self.x_bounds[1]) => return true,
                (_, y) if *y <= f64::from(self.y_bounds[0]) => return true,
                (_, y) if *y >= f64::from(self.y_bounds[1]) => return true,
                _ => {}
            }
        }
        
        false
    }
}
