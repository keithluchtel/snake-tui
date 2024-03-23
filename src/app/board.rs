use ratatui::widgets::canvas::{Painter, Shape};
use crate::app::snake::Snake;
use rand::Rng;
use ratatui::prelude::Color;

#[derive(Clone, Copy, Debug, Default)]
pub enum Direction {
    #[default]
    Up,
    Down,
    Left,
    Right
}

#[derive(Debug)]
struct Target {
    point: [i32; 2]
}

impl Shape for Target {
    fn draw(&self, painter: &mut Painter) {
        painter.paint(self.point[0] as usize, self.point[1] as usize, Color::Blue);
    }
}

impl Target {
    fn new(x_bounds: &[i32; 2], y_bounds: &[i32; 2]) -> Self {
        let x_coord= rand::thread_rng().gen_range((x_bounds[0] + 1)..x_bounds[1]);
        let y_coord= rand::thread_rng().gen_range((y_bounds[0] + 1)..y_bounds[1]);
        Self { point: [x_coord, y_coord] }
    }
}

#[derive(Debug, Default)]
pub struct Board {
    pub x_bounds: [i32; 2],
    pub y_bounds: [i32; 2],
    snake: Snake,
    direction: Direction,
    target: Option<Target>,
}

impl Shape for Board {
    fn draw(&self, painter: &mut Painter) {
        self.snake.draw(painter);
        if let Some(target) = self.target.as_ref() {
            target.draw(painter);
        }
    }
}

impl Board {
    pub fn update_target(&mut self) {
        self.target = Some(Target::new(&self.x_bounds, &self.y_bounds));
    }

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
