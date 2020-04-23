use std::collections::HashMap;

pub struct Matrix<T> {
    grid: HashMap<usize, HashMap<usize, T>>,
}

impl<T> Matrix<T> {
    pub fn new() -> Self {
        let grid = HashMap::new();
        Self { grid }
    }

    pub fn set(&mut self, x: usize, y: usize, elt: T) {
        if !self.grid.contains_key(&x) {
            self.grid.insert(x, HashMap::default());
        }
        let col = self.grid.get_mut(&x).unwrap();

        let cell = col.get_mut(&y);
        match cell {
            None => {
                col.insert(y, elt);
            }
            Some(c) => {
                *c = elt;
            }
        };
    }

    pub fn get(&self, x: usize, y: usize) -> Option<&T> {
        self.grid.get(&x).and_then(|row| row.get(&y))
    }

    pub fn iter(&self) -> Vec<(usize, usize, &T)> {
        self.grid
            .iter()
            .flat_map(|(&x, columns)| columns.iter().map(move |(&y, elt)| (x, y, elt)))
            .collect()
    }
}
