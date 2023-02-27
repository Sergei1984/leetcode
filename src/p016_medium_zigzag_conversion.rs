#![allow(unused)]

pub struct Solution;

impl Solution {
    pub fn convert(s: String, num_rows: i32) -> String {
        println!("rows: {}, input {}", num_rows, s);

        if num_rows == 1 {
            return s;
        }

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
            zigzag.push(vec![None; num_cols]);
        }

        for (idx, char) in s.chars().enumerate() {
            let rem = idx % one_zig_len;
            let zig_start_col = idx / one_zig_len * one_zig_cols;

            if rem < num_rows {
                zigzag[rem][zig_start_col] = Some(char);
            } else {
                let pos = rem - num_rows;

                zigzag[num_rows - pos - 2][zig_start_col + pos + 1] = Some(char);
            }
        }

        Self::print(&zigzag);

        Self::collect(&zigzag)
    }

    fn print(zigzag: &Vec<Vec<Option<char>>>) {
        for row in 0..zigzag.len() {
            for col in 0..zigzag[row].len() {
                let char = zigzag[row][col].unwrap_or('_');

                print!("{}", char);
            }
            println!();
        }
    }

    fn collect(zigzag: &Vec<Vec<Option<char>>>) -> String {
        let mut result = String::new();

        for row in zigzag {
            for char in row {
                if let Some(c) = char {
                    result.push(*c);
                }
            }
        }

        result
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    pub fn case_001() {
        assert_eq!(
            "PAHNAPLSIIGYIR",
            Solution::convert("PAYPALISHIRING".to_string(), 3)
        );
    }

    #[test]
    pub fn case_002() {
        assert_eq!(
            "PINALSIGYAHRPI",
            Solution::convert("PAYPALISHIRING".to_string(), 4)
        );
    }

    #[test]
    pub fn case_003() {
        assert_eq!("A", Solution::convert("A".to_string(), 1));
    }
}
