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
}

const BLACK: types::Color = [0.0, 0.0, 0.0, 1.0];
const WHITE: types::Color = [1.0, 1.0, 1.0, 1.0];

impl<E: Engine> Renderer<E> {
    pub fn new(cell_size: f64, engine: E, grid: Grid, update_interval: Duration) -> Self {
        let width = grid.columns() as f64;
        let height = grid.rows() as f64;
        let window: PistonWindow = WindowSettings::new("Conway's Game of Life", [cell_size * width, cell_size * height])
            .exit_on_esc(true)
            .build()
            .unwrap();

        Self {
            window,
            cell_size,
            engine,
            grid,
            update_interval,
        }
    }

    fn render(&mut self, event: Event) {
        self.window.draw_2d(&event, |context, graphics, _device| {
            clear(WHITE, graphics);

            for (y, row) in self.grid.iter().enumerate() {
                for (x, cell) in row.iter().enumerate() {
                    if cell.is_live() {
                        let rect = rectangle::square(x as f64 * self.cell_size, y as f64 * self.cell_size, self.cell_size);
                        rectangle(BLACK, rect, context.transform, graphics);
                    }
                }
            }
        });
    }

    pub fn start(&mut self) {
        let mut last_update_time = Instant::now() - self.update_interval;

        while let Some(event) = self.window.next() {
            if let Some(_) = event.update_args() {
                let elapsed = last_update_time.elapsed();
                if elapsed >= self.update_interval {
                    self.grid = self.engine.update(&self.grid);
                    last_update_time = Instant::now();
                }
            }

            if let Some(_) = event.render_args() {
                self.render(event);
            }
        }
    }
}
