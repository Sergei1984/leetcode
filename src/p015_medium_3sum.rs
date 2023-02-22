#![allow(unused)]

use core::num;
use std::collections::BTreeMap;

pub struct Solution;

impl Solution {
    pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut map = BTreeMap::<i32, Vec<usize>>::new();

        for (idx, value) in nums.iter().enumerate() {
            let data = map.get_mut(value);
            match data {
                Some(inner) => {
                    inner.push(idx);
                }
                None => {
                    let new_val = vec![idx];
                    map.insert(*value, new_val);
                }
            }
        }

        let mut result = vec![];

        for (idx, value) in nums.iter().enumerate() {
            for (idx2, value2) in nums.iter().enumerate() {
                if (idx != idx2) {
                    let reminder = 0 - value - value2;

                    if let Some(indexes) = map.get(&reminder) {
                        if let Some(idx3) = indexes.iter().find(|i| **i != idx && **i != idx2) {
                            let mut new_trio = vec![nums[idx], nums[idx2], nums[*idx3]];

                            new_trio.sort();

                            if !result.contains(&new_trio) {
                                result.push(new_trio);
                            }
                        }
                    }
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
        let input = vec![-1, 0, 1, 2, -1, -4];

        assert_eq!(
            vec![vec![-1, 0, 1], vec![-1, -1, 2]],
            Solution::three_sum(input)
        );
    }
}
