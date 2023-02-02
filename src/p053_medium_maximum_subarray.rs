#![allow(unused)]

pub struct Solution;

impl Solution {
    pub fn max_sub_array(nums: Vec<i32>) -> i32 {
        Self::max_sub_array_linear(&nums[..])
    }

    pub fn max_sub_array_linear(array: &[i32]) -> i32 {
        let mut max_so_far = i32::min_value();
        let mut max_ending_here = 0;

        for i in 0..array.len() {
            max_ending_here += array[i];
            if max_so_far < max_ending_here {
                max_so_far = max_ending_here;
            }

            if max_ending_here < 0 {
                max_ending_here = 0;
            }
        }

        max_so_far
    }

    pub fn max_sub_array_rec(array: &[i32]) -> i32 {
        if array.len() == 1 {
            return array[0];
        }

        let mid = array.len() / 2;
        let left_sum = Self::max_sub_array_rec(&array[0..mid]);
        let right_sum = Self::max_sub_array_rec(&array[mid..]);

        let mut middle_max_sum = i32::MIN;
        let mut middle_left_sum = 0;

        for i in (0..mid).rev() {
            middle_left_sum += array[i];

            let mut sum = 0;
            for j in mid..array.len() {
                sum += array[j];

                if middle_max_sum < sum + middle_left_sum {
                    middle_max_sum = sum + middle_left_sum;
                }
            }
        }

        left_sum.max(right_sum.max(middle_max_sum))
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    pub fn case_001() {
        assert_eq!(
            Solution::max_sub_array(vec![-2, 1, -3, 4, -1, 2, 1, -5, 4]),
            6
        )
    }

    #[test]
    pub fn case_002() {
        assert_eq!(
            Solution::max_sub_array(vec![-2, -1, -3, -4, -1, -2, -1, -5, -4]),
            -1
        )
    }
}
