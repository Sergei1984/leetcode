#![allow(unused)]

use std::{collections::HashSet, ops::Range};

pub struct Solution;

impl Solution {
    pub fn solve_sudoku(board: &mut Vec<Vec<char>>) {
        let mut sudoku = Sudoku::from_board(board);
        sudoku.print(true);

        let result = Self::try_solve(&mut sudoku);
        if let Some(solved) = result {
            solved.print(true);

            solved.update_board(board);
        }
    }

    fn try_solve(sudoku: &mut Sudoku) -> Option<&Sudoku> {
        loop {
            println!("==============STEP===============");
            let next_pos = sudoku.find_cell_with_single_value();
            if let Some((row, col, digit)) = next_pos {
                sudoku.put(row, col, digit);
                sudoku.print(true);
            } else {
                break;
            }

            println!("");
        }

        if sudoku.empty_cell_count() == 0 {
            return Some(sudoku);
        }

        None
    }
}

#[derive(Default, Debug)]
struct Sudoku {
    field: [[Option<u8>; 9]; 9],
    empty_cell_count: u8,
    row_missing_numbers: [HashSet<u8>; 9],
    col_missing_numbers: [HashSet<u8>; 9],
    quad_missing_numbers: [[HashSet<u8>; 3]; 3],
}

impl Sudoku {
    pub fn from_board(board: &Vec<Vec<char>>) -> Self {
        let mut result = Sudoku::default();
        for row in 0..9 {
            for col in 0..9 {
                let digit = board[row][col];

                result.field[row][col] = if digit == '.' {
                    result.empty_cell_count += 1;
                    None
                } else {
                    Some(digit.to_digit(10).unwrap() as u8)
                };
            }
        }
        result.init_missing_num_data();

        result
    }

    #[inline]
    pub fn rows(&self) -> Range<usize> {
        0..9
    }

    #[inline]
    pub fn cols(&self) -> Range<usize> {
        0..9
    }

    #[inline]
    pub fn quad_rows(&self) -> Range<usize> {
        0..3
    }

    #[inline]
    pub fn quad_cols(&self) -> Range<usize> {
        0..3
    }

    pub fn quad_cell_rows(&self, quad_row: usize, quad_col: usize) -> Range<usize> {
        quad_row * 3..(quad_row * 3) + 3
    }

    pub fn quad_cell_cols(&self, quad_row: usize, quad_col: usize) -> Range<usize> {
        quad_col * 3..(quad_col * 3) + 3
    }

    // Gets quad row and col by cell row and col
    pub fn quad_coords(&self, row: usize, col: usize) -> (usize, usize) {
        (row / 3, col / 3)
    }

    pub fn empty_cell_count(&self) -> u8 {
        self.empty_cell_count
    }

    pub fn find_cell_with_single_value(&self) -> Option<(usize, usize, u8)> {
        for row in self.rows() {
            for col in self.cols() {
                if self.field[row][col].is_none() {
                    let possible_digits = self.cell_possible_digits(row, col);
                    if possible_digits.len() == 1 {
                        return Some((row, col, *possible_digits.iter().next().unwrap()));
                    }
                }
            }
        }
        None
    }

    pub fn print(&self, short: bool) {
        println!("==============================");
        println!("Empty cells: {}", self.empty_cell_count);
        println!("|-----------------------------|");
        for row in self.rows() {
            print!("|");
            for col in self.cols() {
                let val = self.field[row][col];
                if val.is_none() {
                    print!(" . ");
                } else {
                    print!(" {} ", val.unwrap());
                }

                if col % 3 == 2 {
                    print!("|");
                }
            }
            println!("");

            if row % 3 == 2 {
                println!("|-----------------------------|");
            }
        }

        if short {
            return;
        }
        println!("Row missings: ");
        for row in self.rows() {
            println!("R {}: {:?}", row, self.row_missing_numbers[row]);
        }
        println!("Col missings: ");
        for col in self.cols() {
            println!("C {}: {:?}", col, self.col_missing_numbers[col]);
        }
        println!("Quad missings:");
        for quad_row in self.quad_rows() {
            for quad_col in self.quad_cols() {
                println!(
                    "Q ({}, {}): {:?}",
                    quad_row, quad_col, self.quad_missing_numbers[quad_row][quad_col]
                );
            }
        }

        println!("==============================");
    }

    pub fn put(&mut self, row: usize, col: usize, value: u8) -> bool {
        self.field[row][col] = Some(value);
        self.empty_cell_count -= 1;

        let mut value_is_correct = true;

        value_is_correct &= self.col_missing_numbers[col].remove(&value);
        value_is_correct &= self.row_missing_numbers[row].remove(&value);

        let (quad_row, quad_col) = self.quad_coords(row, col);
        value_is_correct &= self.quad_missing_numbers[quad_row][quad_col].remove(&value);

        value_is_correct
    }

    pub fn update_board(&self, board: &mut Vec<Vec<char>>) {
        for row in self.rows() {
            for col in self.cols() {
                let value = self.field[row][col];

                if let Some(digit) = value {
                    board[row][col] = char::from_digit(digit as u32, 10).unwrap();
                } else {
                    board[row][col] = '.';
                }
            }
        }
    }

    fn init_missing_num_data(&mut self) {
        let full_digits = HashSet::<u8>::from([1, 2, 3, 4, 5, 6, 7, 8, 9]);
        // rows
        for row in self.rows() {
            let mut row_digits = full_digits.clone();
            for col in self.cols() {
                let d = self.field[row][col];
                if let Some(v) = d {
                    row_digits.remove(&v);
                }
            }
            self.row_missing_numbers[row] = row_digits;
        }

        // cols
        for col in self.cols() {
            let mut col_digits = full_digits.clone();
            for row in self.rows() {
                let d = self.field[row][col];
                if let Some(v) = d {
                    col_digits.remove(&v);
                }
            }
            self.col_missing_numbers[col] = col_digits;
        }

        // quads
        for quad_row in self.quad_rows() {
            for quad_col in self.quad_cols() {
                let mut quad_digits = full_digits.clone();

                for row in self.quad_cell_rows(quad_row, quad_col) {
                    for col in self.quad_cell_cols(quad_row, quad_col) {
                        let d = self.field[row][col];
                        if let Some(v) = d {
                            quad_digits.remove(&v);
                        }
                    }
                }

                self.quad_missing_numbers[quad_row][quad_col] = quad_digits;
            }
        }
    }

    fn cell_possible_digits(&self, row: usize, col: usize) -> HashSet<u8> {
        let missing_rows = &self.row_missing_numbers[row];
        let missing_col = &self.col_missing_numbers[col];

        let (quad_row, quad_col) = self.quad_coords(row, col);
        let missing_quad = &self.quad_missing_numbers[quad_row][quad_col];

        let missing_row_cols: HashSet<u8> =
            missing_col.intersection(missing_col).map(|a| *a).collect();

        let missing_digits: HashSet<u8> = missing_quad
            .intersection(&missing_row_cols)
            .map(|a| *a)
            .collect();

        missing_digits
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn case_001_single_digit_solving() {
        let mut board = vec![
            vec!['5', '3', '.', '6', '7', '8', '9', '1', '2'],
            vec!['6', '7', '2', '1', '9', '5', '3', '4', '8'],
            vec!['1', '9', '8', '.', '4', '2', '.', '6', '7'],
            vec!['8', '5', '9', '7', '6', '1', '4', '2', '3'],
            vec!['4', '2', '6', '8', '5', '3', '7', '.', '1'],
            vec!['7', '1', '3', '.', '2', '4', '8', '5', '6'],
            vec!['9', '6', '1', '5', '3', '7', '2', '8', '4'],
            vec!['2', '8', '.', '4', '1', '9', '6', '3', '5'],
            vec!['3', '.', '5', '2', '8', '6', '1', '7', '9'],
        ];

        let result = vec![
            vec!['5', '3', '4', '6', '7', '8', '9', '1', '2'],
            vec!['6', '7', '2', '1', '9', '5', '3', '4', '8'],
            vec!['1', '9', '8', '3', '4', '2', '5', '6', '7'],
            vec!['8', '5', '9', '7', '6', '1', '4', '2', '3'],
            vec!['4', '2', '6', '8', '5', '3', '7', '9', '1'],
            vec!['7', '1', '3', '9', '2', '4', '8', '5', '6'],
            vec!['9', '6', '1', '5', '3', '7', '2', '8', '4'],
            vec!['2', '8', '7', '4', '1', '9', '6', '3', '5'],
            vec!['3', '4', '5', '2', '8', '6', '1', '7', '9'],
        ];

        Solution::solve_sudoku(&mut board);

        assert_eq!(board, result);
    }

    #[test]
    fn case_002_leetcode_example() {
        let mut board = vec![
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

        let result = vec![
            vec!['5', '3', '4', '6', '7', '8', '9', '1', '2'],
            vec!['6', '7', '2', '1', '9', '5', '3', '4', '8'],
            vec!['1', '9', '8', '3', '4', '2', '5', '6', '7'],
            vec!['8', '5', '9', '7', '6', '1', '4', '2', '3'],
            vec!['4', '2', '6', '8', '5', '3', '7', '9', '1'],
            vec!['7', '1', '3', '9', '2', '4', '8', '5', '6'],
            vec!['9', '6', '1', '5', '3', '7', '2', '8', '4'],
            vec!['2', '8', '7', '4', '1', '9', '6', '3', '5'],
            vec!['3', '4', '5', '2', '8', '6', '1', '7', '9'],
        ];

        Solution::solve_sudoku(&mut board);

        assert_eq!(board, result);
    }
}
