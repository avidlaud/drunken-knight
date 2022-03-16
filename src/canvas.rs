use crate::action;
use std::fmt;
use std::cmp;

pub trait Grid {
    fn new(rows: usize, cols: usize) -> Self;
    fn size(&self) -> (usize, usize);
    fn get(&self, row: usize, col: usize) -> Option<i8>;
    fn increment(&mut self, row: usize, col: usize);
    fn decrement(&mut self, row: usize, col: usize);
}

#[derive(Debug)]
pub struct FlatGrid {
    rows: usize,
    cols: usize,
    data: Vec<i8>,
}

impl Grid for FlatGrid {
    fn new(rows: usize, cols: usize) -> FlatGrid {
        FlatGrid {
            rows,
            cols,
            data: vec![0; rows * cols],
        }
    }

    fn size(&self) -> (usize, usize) {
        (self.rows, self.cols)
    }

    fn get(&self, row: usize, col: usize) -> Option<i8> {
        if row >= self.rows || col >= self.cols {
            None
        } else {
            Some(self.data[(row * self.cols) + col])
        }
    }

    fn increment(&mut self, row: usize, col: usize) {
        if row < self.rows && col < self.cols {
            self.data[(row * self.cols) + col] += 1
        }
    }

    fn decrement(&mut self, row: usize, col: usize) {
        if row < self.rows && col < self.cols {
            self.data[(row * self.cols) + col] -= 1
        }
    }
}

#[derive(Debug)]
pub struct Canvas<T: Grid> {
    grid: T,
    position: (usize, usize),
}

impl<T: Grid> Canvas<T> {
    pub fn new(grid: T) -> Self {
        let (rows, cols) = grid.size();
        let row_center = rows / 2;
        let col_center = cols / 2;
        Canvas {
            grid,
            position: (row_center, col_center),
        }
    }

    fn increment(&mut self, (row, col): (usize, usize)) {
        self.grid.increment(row, col);
        self.position = (row, col);
    }

    fn decrement(&mut self, (row, col): (usize, usize)) {
        self.grid.decrement(row, col);
        self.position = (row, col);
    }

    // Overflows jumps over the edge of the canvas to the other side.
    fn reposition(&self, move_row: isize, move_col: isize) -> (usize, usize) {
        let (n_row, n_col) = self.grid.size();
        let (r, c) = self.position;

        let m_row = move_row.abs() as usize;
        let m_col = move_col.abs() as usize;

        let new_r = if move_row < 0 { // North move
            // Jumps off grid in the north direction
            if m_row > r {
                r + n_row - m_row
            } else {
                r - m_row
            }
        } else { // South move
            // Jumps off grid in the south direction
            if r + m_row >= n_row {
                r + m_row - n_row
            } else {
                r + m_row
            }
        };
        let new_c = if move_col < 0 { // West move
            // Jumps off grid in the west direction
            if m_col > c {
                c + n_col - m_col
            } else {
                c - m_col
            }
        } else { // East move
            // Jumps off grid in the east direction
            if c + m_col >= n_col {
                c + m_col - n_col
            } else {
                c + m_col
            }
        };
        (new_r, new_c)
    }

    // Does not allow jumps past the edges of the canvas.
    fn bounded_reposition(&self, move_row: isize, move_col: isize) -> (usize, usize) {
        let (n_row, n_col) = self.grid.size();
        let (r, c) = self.position;

        let m_row = move_row.abs() as usize;
        let m_col = move_col.abs() as usize;

        let new_r = if move_row < 0 { // North move
            sub_or_zero(r, m_row)
        } else { // South move
            cmp::min(r + m_row, n_row - 1)
        };
        let new_c = if move_col < 0 { // West move
            sub_or_zero(c, m_col)
        } else { // East move
            cmp::min(c + m_col, n_col - 1)
        };
        (new_r, new_c)
    }

    pub fn simulate(&mut self, a: &action::Action, is_unbounded: bool) {
        if is_unbounded {
            match a {
                action::Action::Increment(j) => match j {
                    action::Jump::N2E1 => self.increment(self.reposition(-2, 1)),
                    action::Jump::N1E2 => self.increment(self.reposition(-1, 2)),
                    action::Jump::S1E2 => self.increment(self.reposition(1, 2)),
                    action::Jump::S2E1 => self.increment(self.reposition(2, 1)),
                    action::Jump::S2W1 => self.increment(self.reposition(2, -1)),
                    action::Jump::S1W2 => self.increment(self.reposition(1, -2)),
                    action::Jump::N1W2 => self.increment(self.reposition(-1, -2)),
                    action::Jump::N2W1 => self.increment(self.reposition(-2, -1)),
                    action::Jump::Still => self.increment((self.position.0, self.position.1)),
                },
                action::Action::Decrement(j) => match j {
                    action::Jump::N2E1 => self.decrement(self.reposition(-2, 1)),
                    action::Jump::N1E2 => self.decrement(self.reposition(-1, 2)),
                    action::Jump::S1E2 => self.decrement(self.reposition(1, 2)),
                    action::Jump::S2E1 => self.decrement(self.reposition(2, 1)),
                    action::Jump::S2W1 => self.decrement(self.reposition(2, -1)),
                    action::Jump::S1W2 => self.decrement(self.reposition(1, -2)),
                    action::Jump::N1W2 => self.decrement(self.reposition(-1, -2)),
                    action::Jump::N2W1 => self.decrement(self.reposition(-2, -1)),
                    action::Jump::Still => self.decrement((self.position.0, self.position.1)),
                },
            }
        } else {
            match a {
                action::Action::Increment(j) => match j {
                    action::Jump::N2E1 => self.increment(self.bounded_reposition(-2, 1)),
                    action::Jump::N1E2 => self.increment(self.bounded_reposition(-1, 2)),
                    action::Jump::S1E2 => self.increment(self.bounded_reposition(1, 2)),
                    action::Jump::S2E1 => self.increment(self.bounded_reposition(2, 1)),
                    action::Jump::S2W1 => self.increment(self.bounded_reposition(2, -1)),
                    action::Jump::S1W2 => self.increment(self.bounded_reposition(1, -2)),
                    action::Jump::N1W2 => self.increment(self.bounded_reposition(-1, -2)),
                    action::Jump::N2W1 => self.increment(self.bounded_reposition(-2, -1)),
                    action::Jump::Still => self.increment((self.position.0, self.position.1)),
                },
                action::Action::Decrement(j) => match j {
                    action::Jump::N2E1 => self.decrement(self.bounded_reposition(-2, 1)),
                    action::Jump::N1E2 => self.decrement(self.bounded_reposition(-1, 2)),
                    action::Jump::S1E2 => self.decrement(self.bounded_reposition(1, 2)),
                    action::Jump::S2E1 => self.decrement(self.bounded_reposition(2, 1)),
                    action::Jump::S2W1 => self.decrement(self.bounded_reposition(2, -1)),
                    action::Jump::S1W2 => self.decrement(self.bounded_reposition(1, -2)),
                    action::Jump::N1W2 => self.decrement(self.bounded_reposition(-1, -2)),
                    action::Jump::N2W1 => self.decrement(self.bounded_reposition(-2, -1)),
                    action::Jump::Still => self.decrement((self.position.0, self.position.1)),
                },
            }
        }
        
    }
}

impl<T: Grid> fmt::Display for Canvas<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let (rows, cols) = self.grid.size();
        write!(f, "Grid:\n")?;
        write!(f, "+{}+\n", "-".repeat(cols))?;
        for row in 0..rows {
            let out: String = (0..cols)
                .map(|col| alphabet(self.grid.get(row, col).unwrap()))
                .collect::<String>();
                // .split("")
                // .map(|s| s.to_string())
                // .collect::<Vec<String>>()
                // .join(" ");
            write!(f, "|{}|\n", out)?;
        }
        write!(f, "+{}+\n", "-".repeat(cols))?;
        Ok(())
    }
}

fn alphabet(i: i8) -> char {
    let chars = [']', '[', '&', '#', '/', '^', '$', '%', '~', '`', ' ', '.', 'o', '+', '=', '*', 'B', 'O', 'X', '@', '\\'];
    let idx = i + 10;
    if idx < 0 {
        'U'
    } else if idx >= chars.len() as i8 {
        'H'
    } else {
        chars[idx as usize]
    }
}

// Subtract the two unsigned integers, return 0 if it would overflow
fn sub_or_zero(a: usize, b: usize) -> usize {
    if b >= a {
        return 0
    }
    a - b
}