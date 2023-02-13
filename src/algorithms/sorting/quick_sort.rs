#![allow(unused)]

pub struct Solution;

impl Solution {
    pub fn sort<T: PartialOrd>(input: &mut Vec<T>) {
        Self::quicksort_rec(input);
    }

    fn quicksort_rec<T: PartialOrd>(input: &mut [T]) {
        if input.len() <= 1 {
            return;
        }

        let q = Self::partition(input);
        Self::quicksort_rec(&mut input[0..q]);
        Self::quicksort_rec(&mut input[q..]);
    }

    fn partition<T: PartialOrd>(input: &mut [T]) -> usize {
        let mut i = -1;

        for j in 0..input.len() - 1 {
            if input[j].le(&input[input.len() - 1]) {
                i += 1;
                input.swap(i as usize, j);
            }
        }
        input.swap((i + 1) as usize, input.len() - 1);

        (i + 1) as usize
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    pub fn case_001() {
        let mut input = vec![13, 19, 9, 5, 12, 8, 7, 4, 21, 2, 6, 11];

        Solution::sort(&mut input);

        assert_eq!(input, vec![2, 4, 5, 6, 7, 8, 9, 11, 12, 13, 19, 21]);
    }

    #[test]
    pub fn case_002() {
        let mut input = vec![13];

        Solution::sort(&mut input);

        assert_eq!(input, vec![13]);
    }

    #[test]
    pub fn case_003() {
        let mut input = vec![13, 12];

        Solution::sort(&mut input);

        assert_eq!(input, vec![12, 13]);
    }
}
