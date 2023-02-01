#![allow(unused)]

pub struct Solution;

impl Solution {
    pub fn sort(input: &mut Vec<i32>) {
        for j in 1..input.len() {
            let elem = input[j];

            let mut i: i32 = (j as i32) - 1;

            while i >= 0 && input[i as usize] > elem {
                input[(i + 1) as usize] = input[i as usize];
                i = i - 1;
            }

            input[(i + 1) as usize] = elem;
        }
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn case_001() {
        let mut input = vec![10, 2, 33, 50, 1];

        Solution::sort(&mut input);

        assert_eq!(input, vec![1, 2, 10, 33, 50]);
    }
}
