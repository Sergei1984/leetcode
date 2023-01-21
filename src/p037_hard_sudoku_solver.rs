#![allow(unused)]

pub struct Solution;

impl Solution {
    pub fn solve_sudoku(board: &mut Vec<Vec<char>>) {
        let mut sudoku = Sudoku::from_board(board);
        sudoku.print();
    }
}

#[derive(Default, Debug)]
struct Sudoku {
    field: [[Option<u8>; 9]; 9],
}

impl Sudoku {
    pub fn from_board(board: &Vec<Vec<char>>) -> Self {
        let mut result = Sudoku::default();
        for row in 0..9 {
            for col in 0..9 {
                let digit = board[row][col];

                result.field[row][col] = if digit == '.' {
                    None
                } else {
                    Some(digit.to_digit(10).unwrap() as u8)
                };
            }
        }

        result
    }

    pub fn print(&self) {
        println!("|-----------------------------|");
        for row in 0..9 {
            print!("|");
            for col in 0..9 {
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
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn case_001() {
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
