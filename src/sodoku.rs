//! Write a function that will solve a 9x9 Sudoku puzzle.
//! The function will take one argument consisting of the 2D puzzle array, with the value 0 representing an unknown square.
//!
//! The Sudokus tested against your function will be "easy" (i.e. determinable; there will be no need to assume and test possibilities on unknowns) and can be solved with a brute-force approach.
//!
//! For Sudoku rules, see the Wikipedia article.
//!
//! // puzzle before
//! puzzle = [
//!     [5,3,0,0,7,0,0,0,0],
//!     [6,0,0,1,9,5,0,0,0],
//!     [0,9,8,0,0,0,0,6,0],
//!     [8,0,0,0,6,0,0,0,3],
//!     [4,0,0,8,0,3,0,0,1],
//!     [7,0,0,0,2,0,0,0,6],
//!     [0,6,0,0,0,0,2,8,0],
//!     [0,0,0,4,1,9,0,0,5],
//!     [0,0,0,0,8,0,0,7,9]
//!   ]
//!
//! sudoku(&mut puzzle);
//! // puzzle after
//!  puzzle == [
//!     [5,3,4,6,7,8,9,1,2],
//!     [6,7,2,1,9,5,3,4,8],
//!     [1,9,8,3,4,2,5,6,7],
//!     [8,5,9,7,6,1,4,2,3],
//!     [4,2,6,8,5,3,7,9,1],
//!     [7,1,3,9,2,4,8,5,6],
//!     [9,6,1,5,3,7,2,8,4],
//!     [2,8,7,4,1,9,6,3,5],
//!     [3,4,5,2,8,6,1,7,9]
//!   ]

use std::collections::HashSet;

#[allow(dead_code)]
pub fn sudoku(puzzle: &mut [[u8; 9]; 9]) {
    sudoku_res(puzzle, 0, 0);
}

pub fn print_board(board: &[[u8; 9]; 9]) {
    println!();
    for row in board {
        println!("{:?}", row);
    }
}

fn sudoku_res(puzzle: &mut [[u8; 9]; 9], i: usize, j: usize) -> bool {
    // Bounds check
    if i >= 9 || j >= 9 {
        return true;
    }

    // Calc the next indexes
    let (new_i, new_j) = match (i, j) {
        (i, j) if j >= 8 => (i + 1, 0),
        (i, j) => (i, j + 1),
    };

    // Don't mess with standing values
    if puzzle[i][j] != 0 {
        return sudoku_res(puzzle, new_i, new_j);
    }

    // Guess and check
    for guess in 1..=9 {
        // guess
        puzzle[i][j] = guess;

        // check
        if !check_board(puzzle) {
            continue;
        }

        // keep going
        let success = sudoku_res(puzzle, new_i, new_j);

        if success {
            return true;
        }
    }

    // reset if wrong
    puzzle[i][j] = 0;
    false
}

fn check_board(puzzle: &[[u8; 9]; 9]) -> bool {
    check_rows(puzzle) && check_cols(puzzle) && check_block(puzzle)
}

fn check_block(puzzle: &[[u8; 9]; 9]) -> bool {
    let mut blocks: Vec<Vec<u8>> = vec![];

    for i in 0..3 {
        for j in 0..3 {
            let mut block: Vec<u8> = vec![];
            for k in 0..3 {
                let i = (i * 3) + k;
                let j = j * 3;
                let block_row = &puzzle[i][j..j + 3];

                block.extend(block_row);
            }
            blocks.push(block);
        }
    }

    blocks
        .into_iter()
        .fold(true, |acc, row| acc & check_line(&row[..]))
}

fn check_cols(puzzle: &[[u8; 9]; 9]) -> bool {
    let columns = (0..9)
        .into_iter()
        .map(|i| puzzle.iter().map(|r| r[i]).collect::<Vec<_>>());

    columns.fold(true, |acc, row| acc & check_line(&row[..]))
}

fn check_rows(puzzle: &[[u8; 9]; 9]) -> bool {
    puzzle.iter().fold(true, |acc, row| acc & check_line(row))
}

fn check_line(line: &[u8]) -> bool {
    let mut seen = HashSet::new();
    for cell in line.iter().filter(|i| **i > 0) {
        if seen.contains(cell) {
            return false;
        }
        seen.insert(*cell);
    }
    true
}

#[cfg(test)]
mod sample_tests {
    use super::sudoku;

    #[test]
    fn puzzle_1() {
        let mut puzzle = [
            [6, 0, 5, 7, 2, 0, 0, 3, 9],
            [4, 0, 0, 0, 0, 5, 1, 0, 0],
            [0, 2, 0, 1, 0, 0, 0, 0, 4],
            [0, 9, 0, 0, 3, 0, 7, 0, 6],
            [1, 0, 0, 8, 0, 9, 0, 0, 5],
            [2, 0, 4, 0, 5, 0, 0, 8, 0],
            [8, 0, 0, 0, 0, 3, 0, 2, 0],
            [0, 0, 2, 9, 0, 0, 0, 0, 1],
            [3, 5, 0, 0, 6, 7, 4, 0, 8],
        ];
        let solution = [
            [6, 1, 5, 7, 2, 4, 8, 3, 9],
            [4, 8, 7, 3, 9, 5, 1, 6, 2],
            [9, 2, 3, 1, 8, 6, 5, 7, 4],
            [5, 9, 8, 4, 3, 2, 7, 1, 6],
            [1, 3, 6, 8, 7, 9, 2, 4, 5],
            [2, 7, 4, 6, 5, 1, 9, 8, 3],
            [8, 4, 9, 5, 1, 3, 6, 2, 7],
            [7, 6, 2, 9, 4, 8, 3, 5, 1],
            [3, 5, 1, 2, 6, 7, 4, 9, 8],
        ];

        sudoku(&mut puzzle);
        assert_eq!(
            puzzle, solution,
            "\nYour solution (left) did not match the correct solution (right)"
        );
    }

    #[test]
    fn puzzle_2() {
        let mut puzzle = [
            [0, 0, 8, 0, 3, 0, 5, 4, 0],
            [3, 0, 0, 4, 0, 7, 9, 0, 0],
            [4, 1, 0, 0, 0, 8, 0, 0, 2],
            [0, 4, 3, 5, 0, 2, 0, 6, 0],
            [5, 0, 0, 0, 0, 0, 0, 0, 8],
            [0, 6, 0, 3, 0, 9, 4, 1, 0],
            [1, 0, 0, 8, 0, 0, 0, 2, 7],
            [0, 0, 5, 6, 0, 3, 0, 0, 4],
            [0, 2, 9, 0, 7, 0, 8, 0, 0],
        ];
        let solution = [
            [9, 7, 8, 2, 3, 1, 5, 4, 6],
            [3, 5, 2, 4, 6, 7, 9, 8, 1],
            [4, 1, 6, 9, 5, 8, 3, 7, 2],
            [8, 4, 3, 5, 1, 2, 7, 6, 9],
            [5, 9, 1, 7, 4, 6, 2, 3, 8],
            [2, 6, 7, 3, 8, 9, 4, 1, 5],
            [1, 3, 4, 8, 9, 5, 6, 2, 7],
            [7, 8, 5, 6, 2, 3, 1, 9, 4],
            [6, 2, 9, 1, 7, 4, 8, 5, 3],
        ];

        sudoku(&mut puzzle);
        assert_eq!(
            puzzle, solution,
            "\nYour solution (left) did not match the correct solution (right)"
        );
    }
}
