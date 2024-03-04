pub mod board {

    #[derive(Copy, Clone, PartialEq)]
    pub enum NonogramCell {
        EMPTY,
        FILLED,
        UNKNOWN
    }

    pub struct Board {
        cells: Vec<Vec<NonogramCell>>,
        row_hints: Vec<Vec<u32>>,
        column_hints: Vec<Vec<u32>>
    }

    impl Board {
        pub fn new(rows: usize, columns: usize, row_hints: Vec<Vec<u32>>, column_hints: Vec<Vec<u32>>) -> Self {
            Board {
                cells: (0..rows).map(|_| (0..columns).map(|_| NonogramCell::UNKNOWN).collect()).collect(),
                row_hints,
                column_hints
            }
        }

        pub fn get_row(&self, row: usize) -> Vec<NonogramCell> {
            self.cells[row].clone()
        }

        pub fn get_row_hint(&self, row: usize) -> Vec<u32> {
            self.row_hints[row].clone()
        }

        pub fn get_column(&self, column: usize) -> Vec<NonogramCell> {
            self.cells.iter().map(|row| row[column]).collect()
        }

        pub fn get_column_hint(&self, column: usize) -> Vec<u32> {
            self.column_hints[column].clone()
        }

        pub fn get_rows_count(&self) -> usize {
            self.cells.len()
        }

        pub fn get_columns_count(&self) -> usize {
            self.cells[0].len()
        }
    }
}