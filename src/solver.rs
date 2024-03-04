pub mod solver {
    use crate::board::board::{Board, NonogramCell};

    pub fn solve(board: Board) -> Board {
        while !(are_rows_filled_correctly(&board) && are_columns_filled_correctly(&board)) {
               
        }
        unimplemented!()
    }

    fn are_rows_filled_correctly(board: &Board) -> bool {
        let rows = board.get_rows_count();

        (0..rows).all(|row| is_row_filled_correctly(board, row))
    }

    fn are_columns_filled_correctly(board: &Board) -> bool {
        let columns = board.get_columns_count();

        (0..columns).all(|column| is_column_filled_correctly(board, column))
    }

    fn is_row_filled_correctly(board: &Board, row: usize) -> bool {
        let row_to_check = board.get_row(row);
        let row_hints = board.get_row_hint(row);

        let filled_cells = count_filled_cells(row_to_check);

        filled_cells == row_hints
    }

    fn is_column_filled_correctly(board: &Board, column: usize) -> bool {
        let column_to_check = board.get_column(column);
        let column_hints = board.get_column_hint(column);

        let filled_cells = count_filled_cells(column_to_check);

        filled_cells == column_hints
    }

    fn count_filled_cells(cells: Vec<NonogramCell>) -> Vec<u32> {
        //TODO - Refactor with map?
        let mut results: Vec<u32> = Vec::new();

        let mut counter: u32 = 0;
        for cell in cells.iter() {
            match &cell {
                NonogramCell::EMPTY => {
                    results.push(counter);
                    counter = 0;
                }
                NonogramCell::FILLED => {
                    counter += 1;
                }
                _ => {}
            }
        }

        if counter > 0 {
            results.push(counter)
        }

        results.remove(0);  //Side effect from the algorithm

        results
    }
}