trait Grid {
    fn new(rows: usize, cols: usize) -> Self;
    fn size(&self) -> (usize, usize);
    fn get(&self, row: usize, col: usize) -> Option<u8>;
    fn increment(&mut self, row: usize, col: usize);
    fn decrement(&mut self, row: usize, col: usize);
}

struct FlatGrid {
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
            Some(self.data[(row * self.rows) + col])
        }
    }

    fn increment(&mut self, row: usize, col: usize) {
        if row < self.rows && col < self.cols {
            self.data[(row * self.rows) + col] += 1
        }
    }

    fn decrement(&mut self, row: usize, col: usize) {
        if row < self.rows && col < self.cols {
            self.data[(row * self.rows) + col] -= 1
        }
    }
}