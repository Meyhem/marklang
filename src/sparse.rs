#[derive(Debug)]
struct Triple<T> {
    row: usize,
    col: usize,
    value: T
}

pub struct SparseMatrix<T> {
    triples: Vec<Triple<T>>
}

impl<T: Copy> SparseMatrix<T> {
    pub fn new() -> Self {
        SparseMatrix {
            triples: Vec::new(),

        }
    }

    pub fn get(&self, row: usize, col: usize) -> Option<T> {
        self.triples.iter().find(|t| t.row == row && t.col == col).map(|t| t.value)
    }

    pub fn put(&mut self, row: usize, col: usize, value: T) {
        let cell = self.triples.iter_mut().find(|t| t.row == row && t.col == col);
        match cell {
            Some(v) => {
                v.value = value;
            },
            None => {
                self.triples.push(Triple {row: row, col: col, value: value});
                self.triples.sort_by(|a, b| a.row.cmp(&b.row));
            }
        }
    }

    pub fn iter_row(&self, row: usize) -> impl Iterator<Item = T> + '_ {
        self.triples
            .iter()
            .skip_while(move |t| t.row != row)
            .take_while(move |t| t.row == row)
            .map(|t| t.value)
    }

    pub fn row_for_each<F>(&mut self, row: usize, mut f: F) where F: FnMut(&mut T)  {
        for t in self.triples.iter_mut() {
            if t.row < row {
                continue;
            }

            if t.row > row {
                break;
            }

            f(&mut t.value);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_row_for_each() {
        #[derive(Clone, Copy, Debug, PartialEq)]
        struct TestType { val: u32 }

        let mut sm: SparseMatrix<TestType> = SparseMatrix::new();
        sm.put(0, 0, TestType { val: 0 });
        sm.put(0, 1, TestType { val: 1 });
        sm.put(1, 0, TestType { val: 2 });
        sm.put(1, 1, TestType { val: 3 });
        sm.row_for_each(0, |v| v.val = 1000);

        assert_eq!(sm.iter_row(0).collect::<Vec<TestType>>(), vec![TestType { val: 1000 }, TestType { val: 1000 }]);
    }

    #[test]
    fn test_put_get() {
        let mut sm: SparseMatrix<u32> = SparseMatrix::new();
        sm.put(0, 0, 55);
        assert_eq!(sm.get(0, 0), Some(55));
        sm.put(0, 0, 42);
        assert_eq!(sm.get(0, 0), Some(42));
    }

    #[test]
    fn test_empty() {
        let sm: SparseMatrix<u32> = SparseMatrix::new();
        assert!(sm.get(0, 0).is_none());
    }

    #[test]
    fn test_row_iter() {
        let mut sm: SparseMatrix<u32> = SparseMatrix::new();
        sm.put(0, 0, 0);
        sm.put(1, 0, 1);

        sm.put(0, 1, 0);
        sm.put(1, 1, 1);

        sm.put(0, 2, 2);

        let row: Vec<u32> = sm.iter_row(0).collect();

        assert_eq!(row, vec![0, 0, 2]);
    }
}