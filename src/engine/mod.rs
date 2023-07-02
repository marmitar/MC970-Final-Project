use crate::cell::Grid;

mod serial;
mod parallel;

pub use serial::SerialEngine;
pub use parallel::ParallelEngine;

pub trait Engine {
    #[must_use]
    fn update(&self, grid: &Grid) -> Grid;
}
