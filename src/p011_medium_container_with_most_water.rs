#![allow(unused)]

use std::collections::BTreeMap;

pub struct Solution;

impl Solution {
    pub fn max_area(height: Vec<i32>) -> i32 {
        let mut height_map = BTreeMap::new();

        // 10   6..11
        //  7   5..7
        //  4   2..8

        // Get the most left and most right indexes of the given height
        for (idx, h) in height.iter().enumerate() {
            height_map
                .entry(*h)
                .and_modify(|e: &mut HeightLeftRight| {
                    (*e).left = (*e).left.min(idx);
                    (*e).right = (*e).right.max(idx);
                })
                .or_insert(HeightLeftRight {
                    left: idx,
                    right: idx,
                });
        }

        if height_map.len() == 1 {
            let (h, bounds) = height_map.iter().next().unwrap();
            return ((bounds.left as i32) - (bounds.right as i32)).abs() * *h;
        }

        let all_keys: Vec<i32> = height_map.keys().rev().map(|i| *i).collect();

        for i in 0..all_keys.len() - 1 {
            let higher_key = all_keys[i];
            let lower_key = all_keys[i + 1];

            let higher_bounds = height_map.get(&higher_key).unwrap().clone();

            height_map.entry(lower_key).and_modify(|e| {
                e.left = e.left.min(higher_bounds.left);
                e.right = e.right.max(higher_bounds.right);
            });
        }

        let mut max_area = 0;

        for h in height_map.keys().rev() {
            let bounds = height_map.get(h).unwrap();

            let area = h * ((bounds.right as i32) - (bounds.left as i32)).abs();

            if area > max_area {
                max_area = area;
            }
        }

        max_area
    }
}

#[derive(Debug, Clone)]
pub struct HeightLeftRight {
    pub left: usize,
    pub right: usize,
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
