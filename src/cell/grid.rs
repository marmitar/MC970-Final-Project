use std::fmt::{self, Display, Formatter, Write};
use std::ops::{Index, IndexMut};

use rand::{Fill, Rng, SeedableRng};
use rand::rngs::SmallRng;

use rayon::iter::IntoParallelIterator;
use rayon::slice::{ParallelSlice, ParallelSliceMut};

use super::Cell;

pub type Iter<'a> =  std::slice::ChunksExact<'a, Cell>;
pub type IterMut<'a> = std::slice::ChunksExactMut<'a, Cell>;
pub type ParIter<'a> = rayon::slice::ChunksExact<'a, Cell>;
pub type ParIterMut<'a> = rayon::slice::ChunksExactMut<'a, Cell>;

/// A 2D matrix representing the current state in Conway's Game of Life.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Grid {
    cells: Box<[Cell]>,
    columns: usize,
}

impl Grid {
    #[inline]
    #[must_use]
    /// Creates a grid of `(rows, columns)` cells in the default state.
    ///
    /// # Panics
    ///
    /// If `rows * columns` overflows an `usize`.
    pub fn new(rows: usize, columns: usize) -> Self {
        Self::new_with(rows, columns, Cell::default())
    }

    #[inline]
    #[must_use]
    /// Creates a grid of `(rows, columns)` cells in the given state.
    ///
    /// # Panics
    ///
    /// If `rows * columns` overflows an `usize`.
    pub fn new_with(rows: usize, columns: usize, cell: Cell) -> Self {
        let cells = rows.checked_mul(columns).expect("number of cells overflows usize");

        Self { cells: vec![cell; cells].into(), columns }
    }

    #[inline]
    #[must_use]
    pub fn random(rows: usize, columns: usize) -> Self {
        let mut rng = SmallRng::from_entropy();
        Self::random_with(rows, columns, &mut rng)
    }

    #[inline]
    #[must_use]
    pub fn random_with<R: Rng + ?Sized>(rows: usize, columns: usize, rng: &mut R) -> Self {
        let mut grid = Self::new(rows, columns);
        grid.try_fill(rng).unwrap();

        grid
    }

    #[inline]
    #[must_use]
    /// Creates an empty grid.
    pub fn empty() -> Self {
        Self::new(0, 0)
    }

    #[must_use]
    /// Creates a grid from a collection of cell slices.
    ///
    /// Returns [`None`] when the slices have different lengths.
    ///
    /// # Example
    ///
    /// ```
    /// # use vida::cell::{Cell, Grid};
    /// #
    /// let grid: Grid = [
    ///     [Cell::Dead, Cell::Live, Cell::Dead],
    ///     [Cell::Live, Cell::Dead, Cell::Live],
    ///     [Cell::Dead, Cell::Live, Cell::Dead],
    ///     [Cell::Dead, Cell::Live, Cell::Dead],
    /// ].into();
    ///
    /// assert_eq!(grid[(1, 2)], Cell::Live);
    /// assert_eq!(grid[1][2], Cell::Live);
    /// ```
    pub fn try_from<T: AsRef<[Cell]>>(grid: impl IntoIterator<Item = T>) -> Option<Self> {
        let mut grid = grid.into_iter().peekable();

        let (expected_rows, _) = grid.size_hint();
        let columns = grid.peek().map_or(0, |row| row.as_ref().len());

        let mut cells = Vec::with_capacity(expected_rows * columns);

        for row in grid {
            if row.as_ref().len() != columns {
                return None
            }

            cells.extend_from_slice(row.as_ref())
        };

        Some(Grid { cells: cells.into(), columns })
    }

    #[inline]
    #[must_use]
    /// The number of rows in the grid.
    pub const fn rows(&self) -> usize {
        if self.columns() > 0 {
            self.cells() / self.columns()
        } else {
            0
        }
    }

    #[inline]
    #[must_use]
    /// The number of columns in each row of the grid.
    pub const fn columns(&self) -> usize {
        self.columns
    }

    #[inline]
    #[must_use]
    /// The shape `(rows, columns)` of the grid.
    pub const fn shape(&self) -> (usize, usize) {
        (self.rows(), self.columns())
    }

    #[inline]
    #[must_use]
    /// The total number of cells in the grid.
    pub const fn cells(&self) -> usize {
        self.cells.len()
    }

    #[inline]
    #[must_use]
    /// A slice over all the cells in the grid, row-major order.
    pub const fn flat(&self) -> &[Cell] {
        &self.cells
    }

    #[inline]
    #[must_use]
    /// A mutable slice over all the cells in the grid, row-major order.
    pub fn flat_mut(&mut self) -> &mut [Cell] {
        &mut self.cells
    }

    #[inline]
    #[must_use]
    /// Returns a reference to a row of cells, without bound checking.
    ///
    /// # Safety
    ///
    /// Calling this method with an out-of-bounds row is undefined behavior.
    pub unsafe fn get_unchecked(&self, row: usize) -> &[Cell] {
        let start = row * self.columns;
        let rest = unsafe { self.cells.get_unchecked(start..) };
        unsafe { rest.get_unchecked(..self.columns) }
    }

    #[inline]
    #[must_use]
    /// Returns a mutable reference to a row of cells, without bound checking.
    ///
    /// # Safety
    ///
    /// Calling this method with an out-of-bounds row is undefined behavior.
    pub unsafe fn get_unchecked_mut(&mut self, row: usize) -> &mut [Cell] {
        let start = row * self.columns;
        let rest = unsafe { self.cells.get_unchecked_mut(start..) };
        unsafe { rest.get_unchecked_mut(..self.columns) }
    }

    #[inline]
    #[must_use]
    /// Returns a reference to a row of cells, without bound checking.
    ///
    /// If the row is out-of-bounds, returns [`None`].
    pub fn get(&self, row: usize) -> Option<&[Cell]> {
        if row.checked_mul(self.columns)? < self.cells() {
            Some(unsafe { self.get_unchecked(row) })
        } else {
            None
        }
    }

    #[inline]
    #[must_use]
    /// Returns a mutable reference to a row of cells, without bound checking.
    ///
    /// If the row is out-of-bounds, returns [`None`].
    pub fn get_mut(&mut self, row: usize) -> Option<&mut [Cell]> {
        if row.checked_mul(self.columns)? < self.cells() {
            Some(unsafe { self.get_unchecked_mut(row) })
        } else {
            None
        }
    }

    #[inline]
    #[must_use]
    /// Returns a reference to a cell, without bound checking.
    ///
    /// # Safety
    ///
    /// Calling this method with an out-of-bounds index is undefined behavior.
    pub unsafe fn get_cell_unchecked(&self, row: usize, col: usize) -> &Cell {
        unsafe { self.get_unchecked(row).get_unchecked(col) }
    }

    #[inline]
    #[must_use]
    /// Returns a mutable reference to a cell, without bound checking.
    ///
    /// # Safety
    ///
    /// Calling this method with an out-of-bounds index is undefined behavior.
    pub unsafe fn get_cell_unchecked_mut(&mut self, row: usize, col: usize) -> &mut Cell {
        unsafe { self.get_unchecked_mut(row).get_unchecked_mut(col) }
    }

    #[inline]
    #[must_use]
    /// Returns a reference to a cell, without bound checking.
    ///
    /// If the index is out-of-bounds, returns [`None`].
    pub fn get_cell(&self, row: usize, col: usize) -> Option<&Cell> {
        self.get(row).and_then(|slice| slice.get(col))
    }

    #[inline]
    #[must_use]
    /// Returns a mutable reference to a cell, without bound checking.
    ///
    /// If the index is out-of-bounds, returns [`None`].
    pub fn get_cell_mut(&mut self, row: usize, col: usize) -> Option<&mut Cell> {
        self.get_mut(row).and_then(|slice| slice.get_mut(col))
    }

    #[inline]
    pub fn iter(&self) -> Iter<'_> {
        self.into_iter()
    }

    #[inline]
    pub fn iter_mut(&mut self) -> IterMut<'_> {
        self.into_iter()
    }
}

impl<T: AsRef<[Cell]>, I: IntoIterator<Item = T>> From<I> for Grid {
    #[inline]
    #[must_use]
    fn from(rows: I) -> Self {
        Grid::try_from(rows).expect("rows with different lengths")
    }
}

impl Index<usize> for Grid {
    type Output = [Cell];

    #[inline]
    #[must_use]
    fn index(&self, row: usize) -> &[Cell] {
        self.get(row).expect("row out of bounds")
    }
}

impl IndexMut<usize> for Grid {
    #[inline]
    #[must_use]
    fn index_mut(&mut self, row: usize) -> &mut [Cell] {
        self.get_mut(row).expect("row out of bounds")
    }
}

impl Index<(usize, usize)> for Grid {
    type Output = Cell;

    #[inline]
    #[must_use]
    fn index(&self, (row, col): (usize, usize)) -> &Cell {
        &self[row][col]
    }
}

impl IndexMut<(usize, usize)> for Grid {
    #[inline]
    #[must_use]
    fn index_mut(&mut self, (row, col): (usize, usize)) -> &mut Cell {
        &mut self[row][col]
    }
}

impl<'a> IntoIterator for &'a Grid {
    type Item = &'a [Cell];
    type IntoIter = Iter<'a>;

    #[inline]
    #[must_use]
    fn into_iter(self) -> Self::IntoIter {
        self.cells.chunks_exact(self.columns)
    }
}

impl<'a> IntoIterator for &'a mut Grid {
    type Item = &'a mut [Cell];
    type IntoIter = IterMut<'a>;

    #[inline]
    #[must_use]
    fn into_iter(self) -> Self::IntoIter {
        self.cells.chunks_exact_mut(self.columns)
    }
}

impl<'a> IntoParallelIterator for &'a Grid {
    type Item = &'a [Cell];
    type Iter = ParIter<'a>;

    #[inline]
    #[must_use]
    fn into_par_iter(self) -> Self::Iter {
        self.cells.par_chunks_exact(self.columns)
    }
}

impl<'a> IntoParallelIterator for &'a mut Grid {
    type Item = &'a mut [Cell];
    type Iter = ParIterMut<'a>;

    #[inline]
    #[must_use]
    fn into_par_iter(self) -> Self::Iter {
        self.cells.par_chunks_exact_mut(self.columns)
    }
}

impl Display for Grid {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        for row in self.iter() {
            for &cell in row {
                write!(f, "{cell}")?
            }
            f.write_char('\n')?
        }
        Ok(())
    }
}

impl Fill for Grid {
    #[inline]
    fn try_fill<R: Rng + ?Sized>(&mut self, rng: &mut R) -> Result<(), rand::Error> {
        for cell in self.cells.iter_mut() {
            *cell = rng.gen()
        }
        Ok(())
    }
}

impl Default for Grid {
    #[inline]
    #[must_use]
    fn default() -> Self {
        Self::empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn convert() {
        let grid: Grid = [
            [Cell::Dead, Cell::Live, Cell::Dead],
            [Cell::Live, Cell::Dead, Cell::Live],
            [Cell::Dead, Cell::Live, Cell::Dead],
            [Cell::Dead, Cell::Live, Cell::Dead],
        ].into();

        assert_eq!(grid.rows(), 4);
        assert_eq!(grid.columns(), 3);
        assert_eq!(grid.shape(), (4, 3));
    }

    #[test]
    pub fn indexing() {
        let cells = [
            [Cell::Dead, Cell::Live, Cell::Dead],
            [Cell::Live, Cell::Dead, Cell::Live],
            [Cell::Dead, Cell::Live, Cell::Dead],
            [Cell::Dead, Cell::Live, Cell::Dead],
        ];
        let grid = Grid::from(cells);

        assert_eq!(grid[0], cells[0]);
        assert_eq!(grid[1], cells[1]);
        assert_eq!(grid[2], cells[2]);
        assert_eq!(grid[3], cells[3]);
    }

    #[test]
    pub fn tuple_indexing() {
        let grid = Grid::try_from([
            [Cell::Dead, Cell::Live, Cell::Dead],
            [Cell::Live, Cell::Dead, Cell::Live],
            [Cell::Dead, Cell::Live, Cell::Dead],
            [Cell::Dead, Cell::Live, Cell::Dead],
        ]).unwrap();

        for row in 0..grid.rows() {
            for col in 0..grid.columns() {
                assert_eq!(grid[row][col], grid[(row, col)])
            }
        }
    }

    #[test]
    pub fn parallel_mutation() {
        use rayon::prelude::*;

        let mut grid = Grid::new_with(5, 10, Cell::Live);
        grid.par_iter_mut().for_each(|row| {
            row.par_iter_mut().for_each(|cell|  {
                if cell.is_live() {
                    *cell = Cell::Dead
                } else {
                    *cell = Cell::Live
                }
            })
        });

        let dead_cells = grid.par_iter()
            .flat_map(|row| row.par_iter())
            .filter(|cell| cell.is_dead())
            .count();

        assert_eq!(dead_cells, grid.cells());
    }
}
