use std::fmt::{self, Display, Formatter, Write};

use rand::Rng;
use rand::distributions::{Distribution, Standard};

mod grid;

pub use grid::{Grid, Iter, IterMut};

/// Represents the state of a single cell in Conways's Game of Life.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
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
    /// Checks if this cell is [`Dead`](Cell::Dead).
    pub const fn is_dead(&self) -> bool {
        matches!(self, Self::Dead)
    }

    #[inline]
    #[must_use]
    /// Checks if this cell is [`Live`](Cell::Live).
    pub const fn is_live(&self) -> bool {
        matches!(self, Self::Live)
    }
}

impl Display for Cell {
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::Dead => f.write_char('D'),
            Self::Live => f.write_char('L'),
        }
    }
}

impl Distribution<Cell> for Standard {
    #[inline]
    #[must_use]
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Cell {
        if <Self as Distribution<bool>>::sample(self, rng) {
            Cell::Live
        } else {
            Cell::Dead
        }
    }
}
