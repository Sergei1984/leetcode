use core::num;
use std::collections::HashMap;

#[allow(dead_code)]
pub struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let map = HashMap::new::<i32, usize>();

        for i in 0..nums.len() {
            let value = target - nums[i];
            if map.contains_key(value) {}
        }

        let mut nums2 = nums.clone();
        nums2.sort();

        let mut higher_idx = nums2.len() - 1;
        let mut lower_idx = 0;

        'h: loop {
            if nums2[higher_idx] > target {
                higher_idx = higher_idx - 1;
                continue;
            }

            let higher = nums2[higher_idx];

            for i in 0..higher_idx {
                let lower = nums2[i];

                if lower + higher > target {
                    higher_idx = higher_idx - 1;
                    continue 'h;
                }

                if lower + higher == target {
                    lower_idx = i;
                    break 'h;
                }
            }
        }

        let lower = nums2[lower_idx];
        let higher = nums2[higher_idx];

        println!("l {}  h {}", lower, higher);

        let mut result = vec![];

        for i in 0..nums.len() {
            if nums[i] == lower || nums[i] == higher {
                result.push(i as i32);
            }
        }

        return result;
    }
}

#[cfg(test)]
mod p1_test {
    use super::Solution;

    #[test]
    fn case1() {
        assert_eq!(Solution::two_sum(vec![2, 7, 11, 15], 9), vec![0, 1]);
    }

    #[test]
    fn case2() {
        assert_eq!(Solution::two_sum(vec![3, 2, 4], 6), vec![1, 2]);
    }

    #[test]
    fn case3() {
        assert_eq!(Solution::two_sum(vec![3, 3], 6), vec![0, 1]);
    }

    #[test]
    fn case4() {
        assert_eq!(Solution::two_sum(vec![0, 4, 3, 0], 0), vec![0, 3]);
    }
}
