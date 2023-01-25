#![allow(unused)]

pub struct Solution;

impl Solution {
    pub fn rotate(matrix: &mut Vec<Vec<i32>>) {
        for level in 0..matrix.len() / 2 {
            let width = matrix.len() - level * 2;
            let perimeter_len = width * 2 + (width - 2) * 2;

            let cells_to_rotate = perimeter_len - width - 1;

            let origin = (level, level);

            println!("R{}", level);
        }
    }

    // moves virtual position
    fn rotate_to(origin: (usize, usize), position: (usize, usize), width: usize) -> (usize, usize) {
        todo!()
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
