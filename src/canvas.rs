use std::fmt;

pub trait Grid {
    fn new(rows: usize, cols: usize) -> Self;
    fn size(&self) -> (usize, usize);
    fn get(&self, row: usize, col: usize) -> Option<u8>;
    fn increment(&mut self, row: usize, col: usize);
    fn decrement(&mut self, row: usize, col: usize);
}

#[derive(Debug)]
pub struct FlatGrid {
    rows: usize,
    cols: usize,
    data: Vec<u8>
}

impl Grid for FlatGrid {
    fn new(rows: usize, cols: usize) -> FlatGrid {
        FlatGrid {
            rows,
            cols,
            data: vec![0; rows*cols],
        }
    }

    fn size(&self) -> (usize, usize) {
        (self.rows, self.cols)
    }

    fn get(&self, row: usize, col: usize) -> Option<u8> {
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
}

impl<T: Grid> Canvas<T> {
    pub fn new(grid: T) -> Self {
        Canvas {
            grid
        }
    }

    fn increment(&mut self, row: usize, col: usize) {
        self.grid.increment(row, col)
    }

    fn decrement(&mut self, row: usize, col: usize) {
        self.grid.decrement(row, col)
    }
}

impl<T: Grid> fmt::Display for Canvas<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Grid:\n")?;
        let (rows, cols) = self.grid.size();
        for row in 0..rows {
            let out: String = (0..cols)
                .map(|col| self.grid.get(row, col).unwrap().to_string())
                .collect();
            write!(f, "{}\n", out)?;
        }
        Ok(())
    }
}