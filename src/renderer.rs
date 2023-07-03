use std::error::Error;
use std::time::{Duration, Instant};

use piston_window::*;
use rayon::prelude::{IntoParallelRefIterator, IndexedParallelIterator, ParallelIterator};

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
            let start = Instant::now();
            self.grid = self.engine.update(&self.grid);
            let elapsed = start.elapsed();
            println!("{:?}", elapsed);

            self.last_update_time = Instant::now();
            Some(())
        } else {
            None
        }
    }

    fn render(&mut self, event: &Event) -> Option<()> {
        self.window.draw_2d(event, |context, graphics, _device| {
            let cell_size = self.cell_size;
            let (sender, receiver) = std::sync::mpsc::channel();

            self.grid.par_iter().enumerate().for_each(move |(row, cells)| {
                cells.par_iter().enumerate().for_each(|(col, cell)| {
                    if cell.is_live() {
                        let (x, y) = (col as f64, row as f64);
                        let rect = rectangle::square(x * cell_size, y * cell_size, cell_size);
                        sender.send(rect).unwrap()
                    }
                })
            });

            clear(WHITE, graphics);
            for rect in receiver.iter() {
                rectangle(BLACK, rect, context.transform, graphics);
            }
        })
    }

    fn next_event(&mut self) -> Option<bool> {
        let event = self.window.next()?;
        let mut updated = false;

        if event.update_args().is_some() {
            updated = self.update().is_some();
        }

        if event.render_args().is_some() {
            self.render(&event);
        }

        Some(updated)
    }

    pub fn next_update(&mut self) -> Option<()> {
        loop {
            match self.next_event() {
                Some(true) => return Some(()),
                Some(false) => continue,
                None => return None
            }
        }
    }

    pub fn start(mut self) {
        while self.next_event().is_some() { }
    }
}
