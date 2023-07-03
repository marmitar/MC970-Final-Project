use std::time::{Duration, Instant};

use clap::Parser;

use vida::cell::Grid;
use vida::engine::{Engine, ParallelEngine, SerialEngine};
use vida::renderer::Renderer;

mod cli;

use cli::{Cli, Mode};

const UPDATE_INTERVAL: Duration = Duration::from_secs(1);

fn main() {
    let cli = Cli::parse();

    let grid = Grid::random(cli.height, cli.width);
    let max_iter = cli.iterations.unwrap_or(usize::MAX);

    if cli.no_render {
        match cli.mode {
            Mode::Serial => run_non_stop(SerialEngine, grid, max_iter),
            Mode::Parallel => run_non_stop(ParallelEngine, grid, max_iter),
        }
    } else {
        match cli.mode {
            Mode::Serial => run_interactive(SerialEngine, grid, cli.cell_size, max_iter),
            Mode::Parallel => run_interactive(ParallelEngine, grid, cli.cell_size, max_iter),
        }
    }
}

fn run_interactive<E: Engine>(engine: E, grid: Grid, cell_size: f64, max_iter: usize) {
    let mut renderer = Renderer::new(cell_size, engine, grid, UPDATE_INTERVAL).unwrap();

    for _ in 0 ..= max_iter {
        if renderer.next_update().is_none() {
            return;
        }
    }
}

fn run_non_stop<E: Engine>(engine: E, mut grid: Grid, max_iter: usize) {
    let start = Instant::now();

    for _ in 0 ..= max_iter {
        grid = engine.update(&grid);
    }

    println!("{:?}", start.elapsed())
}
