use crate::cell::Grid;

mod serial;

pub use serial::SerialEngine;

pub trait Engine {
    #[must_use]
    fn new(rows: usize, columns: usize) -> Self;

    fn set_grid_at(&mut self, grid: Grid, position: (usize, usize));

    #[inline]
    fn set_grid(&mut self, grid: Grid) {
        self.set_grid_at(grid, (0, 0))
    }

    #[must_use]
    fn next_grid(&mut self) -> Grid;

    #[inline]
    #[must_use]
    fn iter(&mut self) -> Iter<'_, Self> {
        Iter { engine: self }
    }
}

#[derive(Debug)]
#[repr(transparent)]
pub struct Iter<'a, E: ?Sized> {
    engine: &'a mut E
}

impl<E: ?Sized + Engine> Iterator for Iter<'_, E> {
    type Item = Grid;

    #[inline]
    #[must_use]
    fn next(&mut self) -> Option<Grid> {
        Some(self.engine.next_grid())
    }

    #[inline]
    #[must_use]
    fn size_hint(&self) -> (usize, Option<usize>) {
        (usize::MAX, None)
    }

    #[inline]
    fn fold<B, F: FnMut(B, Grid) -> B>(self, init: B, mut f: F) -> B {
        let mut accum = init;
        loop {
            accum = f(accum, self.engine.next_grid());
        }
    }
}
