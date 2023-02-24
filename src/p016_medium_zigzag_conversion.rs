#![allow(unused)]

use core::num;
use std::collections::BTreeMap;

pub struct Solution;

impl Solution {
    pub fn convert(s: String, num_rows: i32) -> String {
        let num_rows = num_rows as usize;
        let len = s.len();

        let one_zig_len = (num_rows + num_rows - 2);
        let one_zig_cols = 1 + num_rows - 2;
        let full_zigs = len / one_zig_len;
        let len_reminder = len - full_zigs * one_zig_len;

        let num_cols = full_zigs * one_zig_cols
            + if len_reminder > 0 {
                1 + if len_reminder > num_rows {
                    len_reminder - num_rows
                } else {
                    0
                }
            } else {
                0
            };

        let mut zigzag: Vec<Vec<Option<char>>> = vec![];

        for row in 0..num_rows {
            zigzag.push(Vec::with_capacity(num_cols));
        }

        for (idx, char) in s.chars().enumerate() {
            let rem = idx % one_zig_len;
            let full_zigs = idx / one_zig_len;
            let zig_start_col = full_zigs * one_zig_cols;

            if rem < num_rows {
                zigzag[zig_start_col + 1][rem] = Some(char);
            }
        }

        Self::print(zigzag);

        s
    }

    fn print(zigzag: Vec<Vec<Option<char>>>) {
        for row in 0..zigzag.len() {
            for col in 0..zigzag[row].len() {
                let char = zigzag[row][col].unwrap_or(' ');

                print!("{}", char);
            }
            println!();
        }
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    pub fn case_001() {
        let input = vec![-1, 0, 1, 2, -1, -4];

        assert_eq!(
            "PAHNAPLSIIGYIR",
            Solution::convert("PAYPALISHIRING".to_string(), 3)
        );
    }
}
