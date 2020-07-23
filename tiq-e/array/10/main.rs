use std::collections::HashSet;
use std::time::Instant;

struct Solution;

impl Solution {
    pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
        for i in 0..9 {
            let mut row = HashSet::new();
            let mut column = HashSet::new();
            let mut cube = HashSet::new();
            for j in 0..9 {
                if board[i][j] != '.' {
                    if row.contains(&(board[i][j])) {
                        return false;
                    }
                    row.insert(board[i][j]);
                }

                if board[j][i] != '.' {
                    if column.contains(&(board[j][i])) {
                        return false;
                    }
                    column.insert(board[j][i]);
                }
                let row_idx = 3 * (i / 3) + j / 3;
                let col_idx = 3 * (i % 3) + j % 3;
                if board[row_idx][col_idx] != '.' {
                    if cube.contains(&(board[row_idx][col_idx])) {
                        return false;
                    }
                    cube.insert(board[row_idx][col_idx]);
                }
            }
        }
        true
    }
}

fn main() {
    let arr = vec![
        vec!['5', '3', '.', '.', '7', '.', '.', '.', '.'],
        vec!['6', '.', '.', '1', '9', '5', '.', '.', '.'],
        vec!['.', '9', '8', '.', '.', '.', '.', '6', '.'],
        vec!['8', '.', '.', '.', '6', '.', '.', '.', '3'],
        vec!['4', '.', '.', '8', '.', '3', '.', '.', '1'],
        vec!['7', '.', '.', '.', '2', '.', '.', '.', '6'],
        vec!['.', '6', '.', '.', '.', '.', '2', '8', '.'],
        vec!['.', '.', '.', '4', '1', '9', '.', '.', '5'],
        vec!['.', '.', '.', '.', '8', '.', '.', '7', '9'],
    ];

    let start = Instant::now();
    let result = Solution::is_valid_sudoku(arr);
    let end = start.elapsed().as_micros();
    println!("Duration: {:?}", end);
    println!("Result: {:?}", result);
}
