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

#[allow(dead_code)]
fn sudoku(puzzle: &mut [[u8; 9]; 9]) {
    let _ = puzzle;
    todo!()
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
