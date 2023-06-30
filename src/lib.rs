#[warn(unsafe_op_in_unsafe_fn)]

mod grid;

use std::fmt::Write;

pub use grid::Grid;

/// Represents the state of a single cell in Conways's Game of Life.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Default)]
pub enum Cell {
    #[default]
    /// The cell is currently "unpopulated".
    Dead,
    /// The cell is currently "populated".
    Live,
}

impl Cell {
    #[inline]
    #[must_use]
    pub const fn is_dead(&self) -> bool {
        matches!(self, Self::Dead)
    }

    #[inline]
    #[must_use]
    pub const fn is_live(&self) -> bool {
        matches!(self, Self::Live)
    }
}

impl std::fmt::Display for Cell {
    #[inline]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Cell::Dead => f.write_char('D'),
            Cell::Live => f.write_char('L'),
        }
    }
}
