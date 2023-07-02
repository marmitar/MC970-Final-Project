use std::time::Duration;

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

    match cli.mode {
        Mode::Serial => run(SerialEngine, grid, cli.cell_size),
        Mode::Parallel => run(ParallelEngine, grid, cli.cell_size),
    };
}

fn run<E: Engine>(engine: E, grid: Grid, cell_size: f64) {
    let renderer = Renderer::new(cell_size, engine, grid, UPDATE_INTERVAL).unwrap();

    renderer.start();
}
