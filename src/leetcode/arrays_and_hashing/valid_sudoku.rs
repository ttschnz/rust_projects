pub use super::super::Solution;

use std::collections::HashMap;

impl Solution {
    pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
        let mut board = board;
        let mut checklist: HashMap<char, bool> = HashMap::new();
        (1..=9).for_each(|i| {
            checklist.insert(char::from_digit(i, 10).unwrap(), true);
        });

        // Each row must contain the digits 1-9 without repetition.
        let rows = board.iter().fold(true, |valid, row| {
            if !Solution::check_row(row, &mut checklist.clone()) {
                false
            } else {
                valid
            }
        });
        Solution::rotate_vector(&mut board);
        // Each column must contain the digits 1-9 without repetition.
        let columns = board.iter().fold(true, |valid, row| {
            if !Solution::check_row(row, &mut checklist.clone()) {
                false
            } else {
                valid
            }
        });

        // Each of the nine 3 x 3 sub-boxes of the grid must contain the digits 1-9 without repetition.
    }
    fn check_row(row: &Vec<char>, row_checklist: &mut HashMap<char, bool>) -> bool {
        let mut row_valid = true;
        row.iter().for_each(|cell| match cell {
            '.' => {}
            _ => {
                if row_checklist[cell] {
                    row_checklist.insert(cell.clone(), false);
                } else {
                    row_valid = false;
                }
            }
        });
        row_valid
    }

    fn rotate_vector<T>(matrix: &mut Vec<Vec<T>>)
    where
        T: Copy,
    {
        let n = matrix.len();

        // Transpose the matrix
        for i in 0..n {
            for j in i..n {
                let temp = matrix[i][j];
                matrix[i][j] = matrix[j][i];
                matrix[j][i] = temp;
            }
        }

        // Reverse each row of the transposed matrix
        for i in 0..n {
            matrix[i].reverse();
        }
    }

    fn sub_box<T>(matrix: &Vec<Vec<T>>, x: usize, y: usize, size: usize) -> Vec<Vec<T>>
    where
        T: Clone, Copy;
    {
        let mut out: Vec<Vec<T>> = vec![vec![]; size];
        for i in x..size {
            for j in y..size {
                out[i - size][j - size] = matrix[i][j]
            }
        }
        out
    }
}
