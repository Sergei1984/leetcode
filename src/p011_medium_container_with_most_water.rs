#![allow(unused)]

use std::collections::BTreeMap;

pub struct Solution;

impl Solution {
    pub fn max_area(height: Vec<i32>) -> i32 {
        let mut max_area = 0;

        for i in 0..height.len() {
            for j in (i + 1..height.len()).rev() {
                let h = height[i].min(height[j]);
                let area = ((j - i) as i32) * h;

                max_area = max_area.max(area);
            }
        }

        max_area
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn case_001() {
        assert_eq!(Solution::max_area(vec![1, 8, 6, 2, 5, 4, 8, 3, 7]), 49);
    }

    #[test]
    fn case_002() {
        assert_eq!(Solution::max_area(vec![1, 1]), 1);
    }
}
