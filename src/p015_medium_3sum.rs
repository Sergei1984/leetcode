#![allow(unused)]

use std::collections::BTreeMap;

pub struct Solution;

impl Solution {
    pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
        if nums.len() == 0 {
            return vec![];
        }

        let mut map = BTreeMap::<i32, usize>::new();

        for (idx, value) in nums.iter().enumerate() {
            let data = map.get_mut(value);
            match data {
                Some(inner) => {
                    *inner += 1;
                }
                None => {
                    map.insert(*value, 1);
                }
            }
        }

        let mut result = vec![];

        let sorted_values: Vec<i32> = map.keys().map(|k| *k).collect();

        let min_key = *sorted_values.first().unwrap();
        let max_key = *sorted_values.last().unwrap();

        for i in 0..sorted_values.len() {
            for j in i..sorted_values.len() {
                let value1 = sorted_values[i];
                let value2 = sorted_values[j];

                let value3 = 0 - value1 - value2;

                if map.get(&value3).is_some() {
                    let mut v2_count = 1;
                    let mut v3_count = 1;

                    if value2 == value1 {
                        v2_count += 1;
                    }

                    if value1 == value3 {
                        v3_count += 1;
                    }

                    if value2 == value3 {
                        v3_count += 1;
                    }

                    if let Some(actual_count) = map.get(&value2) {
                        if *actual_count < v2_count {
                            continue;
                        }
                    } else {
                        panic!("value2 can't be found in map");
                    }

                    if let Some(actual_count) = map.get(&value3) {
                        if *actual_count < v3_count {
                            continue;
                        }
                    } else {
                        panic!("value3 can't be found in map");
                    }

                    let mut new_trio = vec![value1, value2, value3];

                    new_trio.sort();

                    if !result.contains(&new_trio) {
                        result.push(new_trio);
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
            vec![vec![-1, -1, 2], vec![-1, 0, 1]],
            Solution::three_sum(input)
        );
    }
}
