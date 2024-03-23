mod board;
mod snake;

use std::io::Stdout;
use crossterm::event;
use crossterm::event::{Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::backend::CrosstermBackend;
use ratatui::{Frame, Terminal};
use ratatui::buffer::Buffer;
use ratatui::layout::{Alignment, Rect};
use ratatui::prelude::{Marker, Widget};
use ratatui::symbols::border;
use ratatui::widgets::{Block, Borders};
use ratatui::widgets::block::Title;
use ratatui::widgets::canvas::Canvas;
use crate::app::board::Board;
use crate::app::snake::Snake;

#[derive(Debug, Default)]
pub struct App {
    exit: bool,
    board: Board,
}
impl App {
    pub fn run(&mut self, terminal: &mut Terminal<CrosstermBackend<Stdout>>) -> anyhow::Result<()> {

        while !self.exit {
            terminal.draw(|frame| self.render_frame(frame))?;
            self.handle_events()?;
        }
        Ok(())
    }

    fn render_frame(&mut self, frame: &mut Frame) {
        frame.render_widget(self, frame.size());
    }

    fn handle_events(&mut self) -> anyhow::Result<()> {
        if event::poll(std::time::Duration::from_millis(500))? {
            match event::read()? {
                Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                    self.handle_key_event(key_event)
                }
                _ => {}
            }
        }

        self.handle_tick_event();
        Ok(())
    }

    fn handle_key_event(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Char('q') => self.exit(),
            _ => {}
        }
    }

    fn handle_tick_event(&mut self) {
        self.board.process_tick();
        
        if self.board.check_collisions() {
            self.exit();
        }
    }

    fn exit(&mut self) {
        self.exit = true;
    }
}

impl Widget for &mut App {
    fn render(self, area: Rect, buf: &mut Buffer) where Self: Sized {
        let title = Title::from(" Snake ");
        let block = Block::default()
            .title(title.alignment(Alignment::Center))
            .borders(Borders::ALL)
            .border_set(border::THICK);

        self.board.x_bounds = [0, i32::from(area.width)];
        self.board.y_bounds = [0, i32::from(2 * area.height)];

        Canvas::default()
            .marker(Marker::HalfBlock)
            .x_bounds([self.board.x_bounds[0] as f64, self.board.x_bounds[1] as f64])
            .y_bounds([self.board.y_bounds[0] as f64, self.board.y_bounds[1] as f64])
            .paint(|ctx| {
                ctx.draw(&self.board);
            })
            .block(block)
            .render(area, buf);
    }
}
