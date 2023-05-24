pub use super::super::Solution;

use std::collections::HashMap;

impl Solution {
    pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
        let mut board = board;
        let mut checklist: HashMap<char, bool> = HashMap::new();
        (1..=9).for_each(|i| {
            checklist.insert(char::from_digit(i, 10).unwrap(), true);
        });
        let mut valid = true;
        // Each row must contain the digits 1-9 without repetition.
        valid = board.iter().fold(true, |acc_valid, row| {
            Solution::check_row(row, &mut checklist.clone()) && acc_valid
        }) && valid;
        Solution::rotate_vector(&mut board);
        // Each column must contain the digits 1-9 without repetition.
        valid = board.iter().fold(true, |acc_valid, row| {
            Solution::check_row(row, &mut checklist.clone()) && acc_valid
        }) && valid;

        // Each of the nine 3 x 3 sub-boxes of the grid must contain the digits 1-9 without repetition.
        for x in 0..3 {
            for y in 0..3 {
                let row = Solution::sub_box(&board, x * 3, y * 3, 3)
                    .iter()
                    .flatten()
                    .map(|n| n.clone())
                    .collect::<Vec<char>>();
                valid = Solution::check_row(&row, &mut checklist.clone()) && valid
            }
        }
        valid
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
        T: Clone + Copy + Default,
    {
        let mut out: Vec<Vec<T>> = vec![vec![Default::default(); size]; size];
        for i in x..size + x {
            for j in y..size + y {
                out[i - x][j - y] = matrix[i][j]
            }
        }
        out
    }
}

#[cfg(test)]
mod test {
    use super::Solution;
    #[test]
    fn sub_box_works() {
        let matrix = vec![
            vec![11, 12, 13, 14, 15, 16, 17, 18, 19],
            vec![21, 22, 23, 24, 25, 26, 27, 28, 29],
            vec![31, 32, 33, 34, 35, 36, 37, 38, 39],
            vec![41, 42, 43, 44, 45, 46, 47, 48, 49],
            vec![51, 52, 53, 54, 55, 56, 57, 58, 59],
            vec![61, 62, 63, 64, 65, 66, 67, 68, 69],
            vec![71, 72, 73, 74, 75, 76, 77, 78, 79],
            vec![81, 82, 83, 84, 85, 86, 87, 88, 89],
            vec![91, 92, 93, 94, 95, 96, 97, 98, 99],
        ];

        assert_eq!(
            Solution::sub_box(&matrix, 0, 0, 3),
            vec![vec![11, 12, 13], vec![21, 22, 23], vec![31, 32, 33]]
        );
        assert_eq!(
            Solution::sub_box(&matrix, 3, 3, 3),
            vec![vec![44, 45, 46], vec![54, 55, 56], vec![64, 65, 66]]
        );
        assert_eq!(
            Solution::sub_box(&matrix, 0, 3, 3),
            vec![vec![14, 15, 16], vec![24, 25, 26], vec![34, 35, 36]]
        );
    }
    #[test]
    fn runs_through() {
        let matrix = vec![vec!['.'; 9]; 9];
        assert_eq!(Solution::is_valid_sudoku(matrix), true);
    }
    #[test]
    fn sees_duplicate() {
        let matrix = vec![vec!['1'; 9]; 9];
        assert_eq!(Solution::is_valid_sudoku(matrix), false);
    }

    #[test]
    fn sees_duplicate_in_row() {
        let matrix = vec![vec!['.'; 9]; 9];
        for i in 0..9 {
            let mut matrix = matrix.clone();
            matrix[i] = vec!['1'; 9];
            assert_eq!(Solution::is_valid_sudoku(matrix), false);
        }
    }
    #[test]
    fn sees_duplicate_in_col() {
        let matrix = vec![vec!['.'; 9]; 9];
        for i in 0..9 {
            let mut matrix = matrix.clone();
            for j in 0..9 {
                matrix[j][i] = '1';
            }
            assert_eq!(Solution::is_valid_sudoku(matrix), false);
        }
    }
    #[test]
    fn sees_duplicate_in_sub_box() {
        let matrix = vec![vec!['.'; 9]; 9];
        for i in 0..3 {
            for j in 0..3 {
                let mut matrix = matrix.clone();
                for x in i..=i + 3 {
                    for y in j..=j + 3 {
                        matrix[x][y] = '1';
                    }
                }
                assert_eq!(Solution::is_valid_sudoku(matrix), false);
            }
        }
    }
}
