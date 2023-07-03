use clap::{Parser, ValueEnum};

const CELL_SIZE: f64 = 2.0;
const GRID_WIDTH: usize = 768;
const GRID_HEIGHT: usize = 432;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    /// What mode to run the program in.
    #[arg(value_enum)]
    pub mode: Mode,

    /// Size of the cell.
    #[arg(short, long, default_value_t = CELL_SIZE)]
    pub cell_size: f64,

    /// Width of the grid.
    #[arg(short, long, default_value_t = GRID_WIDTH)]
    pub width: usize,

    /// Height of the grid.
    #[arg(short = 'H', long, default_value_t = GRID_HEIGHT)]
    pub height: usize,

    /// Open window for rendering the game.
    #[arg(short = 'r', long, default_value_t = false)]
    pub no_render: bool,

    /// Maximum number of iterations.
    #[arg(short, long, required = false)]
    pub iterations: Option<usize>,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum Mode {
    /// Serial Mode
    Serial,
    /// Parallel Mode
    Parallel,
}
