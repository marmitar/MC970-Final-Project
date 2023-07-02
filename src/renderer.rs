use std::error::Error;
use std::time::{Duration, Instant};

use piston_window::*;

use crate::cell::Grid;
use crate::engine::Engine;

pub struct Renderer<E> {
    window: PistonWindow,
    cell_size: f64,
    engine: E,
    grid: Grid,
    update_interval: Duration, // The duration of the delay between updates
    last_update_time: Instant,
}

const BLACK: types::Color = [0.0, 0.0, 0.0, 1.0];
const WHITE: types::Color = [1.0, 1.0, 1.0, 1.0];

impl<E: Engine> Renderer<E> {
    pub fn new(cell_size: f64, engine: E, grid: Grid, update_interval: Duration) -> Result<Self, Box<dyn Error>> {
        let (width, height) = (grid.columns() as f64, grid.rows() as f64);
        let window = WindowSettings::new("Conway's Game of Life", [cell_size * width, cell_size * height])
            .exit_on_esc(true)
            .build()?;

        let last_update_time = Instant::now() - update_interval;

        Ok(Self { window, cell_size, engine, grid, update_interval, last_update_time })
    }

    fn update(&mut self) -> Option<()> {
        let elapsed = self.last_update_time.elapsed();

        if elapsed >= self.update_interval {
            self.grid = self.engine.update(&self.grid);
            self.last_update_time = Instant::now();
            Some(())
        } else {
            None
        }
    }

    fn render(&mut self, event: &Event) -> Option<()> {
        self.window.draw_2d(event, |context, graphics, _device| {
            clear(WHITE, graphics);

            for (y, row) in self.grid.iter().enumerate() {
                for (x, cell) in row.iter().enumerate() {
                    if cell.is_live() {
                        let rect = rectangle::square(x as f64 * self.cell_size, y as f64 * self.cell_size, self.cell_size);
                        rectangle(BLACK, rect, context.transform, graphics);
                    }
                }
            }
        })
    }

    fn next_event(&mut self) -> Option<Event> {
        if let Some(event) = self.window.next() {
            if event.update_args().is_some() {
                self.update();
            }

            if event.render_args().is_some() {
                self.render(&event);
            }

            Some(event)
        } else {
            None
        }
    }

    pub fn start(mut self) {
        while self.next_event().is_some() { }
    }
}
