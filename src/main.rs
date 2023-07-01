use clap::Parser;
use vida::cell::{Grid};
use vida::engine::{Engine, ParallelEngine, SerialEngine};
use vida::renderer::Renderer;
use vida::cli::{self, Mode};

use std::time::Duration;

const UPDATE_INTERVAL: Duration = Duration::from_secs(1);

fn main() {
    let cli = cli::Cli::parse();

    let grid = Grid::random(cli.height, cli.width);

    match cli.mode {
        Mode::Serial => run(SerialEngine, grid, cli.cell_size),
        Mode::Paralell => run(ParallelEngine, grid, cli.cell_size),
    };
}

fn run<E: Engine>(engine: E, grid: Grid, cell_size: f64) {
    let mut renderer = Renderer::new(cell_size, engine, grid, UPDATE_INTERVAL);

    renderer.start();
}
