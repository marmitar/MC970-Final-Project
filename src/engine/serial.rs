use crate::cell::{Cell, Grid};

use super::Engine;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
#[repr(transparent)]
pub struct SerialEngine {
    grid: Grid,
}

impl SerialEngine {
    #[must_use]
    fn next_cell_at(&self, row: usize, col: usize) -> Cell {
        let start_row = row.saturating_sub(1);
        let start_col = col.saturating_sub(1);

        let mut live_cells = 0;

        for i in start_row..start_row+3 {
            for j in start_col..start_col+3 {
                if (i, j) != (row, col) && self.grid.get_cell(i, j) == Some(&Cell::Live) {
                    live_cells += 1
                }
            }
        }

        if live_cells == 3 || (live_cells == 2 && self.grid[row][col].is_live()) {
            Cell::Live
        } else {
            Cell::Dead
        }

    }

    #[must_use]
    fn prepare_next_grid(&self) -> Grid {
        let mut next = Grid::new_with(self.grid.rows(), self.grid.columns(), Cell::Dead);

        for (row, cells) in next.iter_mut().enumerate() {
            for (col, cell) in cells.iter_mut().enumerate() {
                if self.next_cell_at(row, col).is_live() {
                    *cell = Cell::Live
                }
            }
        }

        next
    }

    fn update_grid(&mut self) -> Grid {
        let next = self.prepare_next_grid();
        std::mem::replace(&mut self.grid, next)
    }
}

impl Engine for SerialEngine {
    #[inline]
    #[must_use]
    fn new(rows: usize, columns: usize) -> Self {
        Self { grid: Grid::new(rows, columns) }
    }

    #[inline]
    fn set_grid_at(&mut self, grid: Grid, (start_row, start_col): (usize, usize)) {
        for (row, cells) in grid.iter().enumerate() {
            for (col, &cell) in cells.iter().enumerate() {
                if let Some(target) = self.grid.get_cell_mut(start_row + row, start_col + col) {
                    *target = cell
                }
            }
        }
    }

    #[inline]
    fn next_grid(&mut self) -> Grid {
        self.update_grid()
    }
}
