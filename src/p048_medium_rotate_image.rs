#![allow(unused)]

use std::slice::Windows;

pub struct Solution;

impl Solution {
    pub fn rotate(matrix: &mut Vec<Vec<i32>>) {
        if matrix.len() == 1 {
            return;
        }

        for level in 0..matrix.len() / 2 {
            let width = matrix.len() - level * 2;

            for sqr_start_col in 0..width - 1 {
                let mut rotating_value = matrix[level][sqr_start_col + level];

                let mut pos_row = level;
                let mut pos_col = sqr_start_col + level;

                for _ in 0..4 {
                    let next_row = pos_col;
                    let next_col = matrix.len() - 1 - pos_row;

                    let tmp = matrix[next_row][next_col];
                    matrix[next_row][next_col] = rotating_value;
                    rotating_value = tmp;
                    pos_row = next_row;
                    pos_col = next_col;
                }
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn case_001() {
        let mut input = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];

        Solution::rotate(&mut input);

        assert_eq!(input, vec![vec![7, 4, 1], vec![8, 5, 2], vec![9, 6, 3]]);
    }

    #[test]
    fn case_002() {
        let mut input = vec![
            vec![5, 1, 9, 11],
            vec![2, 4, 8, 10],
            vec![13, 3, 6, 7],
            vec![15, 14, 12, 16],
        ];

        Solution::rotate(&mut input);

        assert_eq!(
            input,
            vec![
                vec![15, 13, 2, 5],
                vec![14, 3, 4, 1],
                vec![12, 6, 8, 9],
                vec![16, 7, 10, 11]
            ]
        );
    }

    #[test]
    fn case_003() {
        let mut input = vec![vec![1]];

        Solution::rotate(&mut input);

        assert_eq!(input, vec![vec![1]]);
    }
}
