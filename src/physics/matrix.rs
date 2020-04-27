use crate::physics::size::Size;

pub struct Matrix<T> {
    grid: Vec<Vec<Option<T>>>,
    size: Size,
}

impl<T: Clone> Matrix<T> {
    pub fn new(size: Size) -> Self {
        let mut grid: Vec<Vec<Option<T>>> = vec![];
        for _i in 0..size.width {
            grid.push(vec![None; size.height as usize]);
        }
        Self { grid, size }
    }

    pub fn get(&self, x: usize, y: usize) -> Option<&T> {
        if x < self.size.width as usize && y < self.size.height as usize {
            self.grid[x][y].as_ref()
        } else {
            None
        }
    }

    pub fn set(&mut self, x: usize, y: usize, elt: T) {
        if x < self.size.width as usize && y < self.size.height as usize {
            self.grid[x][y] = Some(elt);
        }
    }

    #[allow(dead_code)]
    pub fn reset(&mut self, x: usize, y: usize) {
        if x < self.size.width as usize && y < self.size.height as usize {
            self.grid[x][y] = None;
        }
    }

    pub fn iter(&self) -> Vec<(usize, usize, &T)> {
        let mut result = vec![];
        for (x, column) in self.grid.iter().enumerate() {
            for (y, cell) in column.iter().enumerate() {
                if let Some(elt) = cell {
                    result.push((x, y, elt));
                }
            }
        }
        result
    }
}
