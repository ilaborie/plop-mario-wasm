use std::collections::HashMap;

pub struct Matrix<T> {
    grid: HashMap<usize, HashMap<usize, T>>,
}

impl<T: Clone> Matrix<T> {
    pub fn new() -> Self {
        let grid = HashMap::new();
        Self { grid }
    }

    pub fn set(&mut self, x: usize, y: usize, elt: T) {
        if !self.grid.contains_key(&x) {
            self.grid.insert(x, HashMap::default());
        }
        let col = self.grid.get_mut(&x).unwrap();

        // log(&format!("Set ({}, {}) = {}", x, y, elt).to_string());
        *col.entry(y).or_insert(elt.clone()) = elt.clone();
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
