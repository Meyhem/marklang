struct Triple<'a, T> {
    row: usize,
    col: usize,
    value: &'a T
}

pub struct SparseMatrix<'a, T> {
    triples: Vec<Triple<'a, T>>,
    max_rows: usize,
    max_cols: usize
}

impl<'a, T> SparseMatrix<'a, T> {
    pub fn new() -> Self {
        SparseMatrix {
            triples: Vec::new(),
            max_rows: 0,
            max_cols: 0
        }
    }

    pub fn rows(&self) -> usize {
        self.max_rows
    }

    pub fn cols(&self) -> usize {
        self.max_cols
    }

    pub fn get(&self, row: usize, col: usize) -> Option<&T> {
        self.triples.iter().find(|t| t.row == row && t.col == col).map(|t| t.value)
    }

    pub fn put(&mut self, row: usize, col: usize, value: &'a T) {
        if self.max_rows < row {
            self.max_rows = row;
        }
        if self.max_cols < col {
            self.max_cols = col;
        }

        let cell = self.triples.iter_mut().find(|t| t.row == row && t.col == col);
        match cell {
            Some(v) => {
                v.value = value;
            },
            None => {
                self.triples.push(Triple {row: row, col: col, value: value});
            }
        }
    }

    pub fn iter_row(&self, row: usize) -> impl Iterator<Item = &T> {
        if row >= self.max_rows {
            panic!("Row index out of range");
        }

        self.triples.iter().filter(move |t| t.row == row).map(|t| t.value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_put_get() {
        let mut sm: SparseMatrix<u32> = SparseMatrix::new();
        sm.put(0, 0, &55);
        assert_eq!(sm.get(0, 0), Some(&55));
        sm.put(0, 0, &42);
        assert_eq!(sm.get(0, 0), Some(&42));
    }

    #[test]
    fn test_empty() {
        let sm: SparseMatrix<u32> = SparseMatrix::new();
        assert!(sm.get(0, 0).is_none());
    }

    #[test]
    fn test_row_iter() {
        let mut sm: SparseMatrix<u32> = SparseMatrix::new();
        sm.put(0, 0, &0);
        sm.put(1, 0, &1);

        sm.put(0, 1, &0);
        sm.put(1, 1, &1);

        sm.put(0, 2, &2);

        let row: Vec<&u32> = sm.iter_row(0).collect();

        assert_eq!(row, vec![&0, &0, &2]);
    }

    #[test]
    #[should_panic(expected = "Row index out of range")]
    fn test_row_iter_panic() {
        let sm: SparseMatrix<u32> = SparseMatrix::new();
        sm.iter_row(42);
    }

}