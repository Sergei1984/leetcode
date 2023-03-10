#![allow(unused)]

pub struct Solution;

impl Solution {
    pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        let result = Self::find_median_rec(&nums1, &nums2, 0, 0);
        result
    }

    fn find_median_rec(
        left: &[i32],
        right: &[i32],
        extra_smaller: usize,
        extra_bigger: usize,
    ) -> f64 {
        let bigger;
        let smaller;

        if left.len() > right.len() {
            bigger = left;
            smaller = right;
        } else {
            bigger = right;
            smaller = left;
        }

        if bigger.len() <= 2 {
            let combined_array = Self::merge_join(bigger, smaller);
            return Self::find_median_value(&combined_array, extra_smaller, extra_bigger);
        }

        let idx = bigger.len() / 2;
        let median = bigger[idx];
        let b_smaller_cnt = idx + 1;
        let b_bigger_cnt = bigger.len() - idx;

        let smaller_idx = if smaller.len() == 0 {
            None
        } else {
            Some(Self::find_index_less_or_equal(smaller, median))
        };

        let smaller_median = smaller_idx.map(|i| smaller[i]);

        let other_smaller_cnt = smaller_idx
            .zip(smaller_median)
            .map(|(idx, m)| {
                if m <= median {
                    return idx + 1;
                } else {
                    return idx;
                }
            })
            .unwrap_or(0);
        let other_bigger_cnt = smaller.len() - other_smaller_cnt;

        let b = b_bigger_cnt + extra_bigger + other_bigger_cnt;
        let s = b_smaller_cnt + other_smaller_cnt + extra_smaller;

        // [1 2 7]
        // [3 4]
        if b < s {
            let next_bigger = &bigger[0..idx + 1];
            return Self::find_median_rec(
                next_bigger,
                smaller,
                extra_smaller,
                extra_bigger + bigger.len() - next_bigger.len(),
            );
        } else {
            let next_bigger = &bigger[idx..];
            return Self::find_median_rec(
                next_bigger,
                smaller,
                extra_smaller + bigger.len() - next_bigger.len(),
                extra_bigger,
            );
        }
    }

    // Find index of value less or equals to specified using binary search.
    pub fn find_index_less_or_equal(array: &[i32], value: i32) -> usize {
        let mut lower = 0;
        let mut higher = array.len() - 1;

        loop {
            if lower == higher {
                return lower;
            }

            if lower + 1 == higher {
                if array[higher] <= value {
                    return higher;
                } else {
                    return lower;
                }
            }

            let mid = (lower + higher) / 2;

            if array[mid] == value {
                return mid;
            }

            if array[mid] < value {
                lower = mid;
            } else {
                higher = mid;
            }
        }
    }

    pub fn merge_join(v1: &[i32], v2: &[i32]) -> Vec<i32> {
        let mut result = v1.to_vec();
        result.extend(v2);

        result.sort();

        result
    }

    pub fn find_median_value(array: &[i32], extra_small: usize, extra_large: usize) -> f64 {
        let is_odd = (array.len() + extra_large + extra_small) % 2 == 1;

        let middle = (array.len() + extra_large + extra_small) / 2;
        let middle_index = middle - extra_small;

        if is_odd {
            return array[middle_index] as f64;
        } else {
            return ((array[middle_index - 1] + array[middle_index]) as f64) / 2.;
        }
    }
}

mod test {
    use super::Solution;

    #[test]
    fn case_001() {
        assert_eq!(Solution::find_median_sorted_arrays(vec![1, 3], vec![2]), 2.);
    }

    #[test]
    fn case_002() {
        assert_eq!(
            Solution::find_median_sorted_arrays(vec![1, 2], vec![3, 4]),
            2.5
        );
    }

    #[test]
    fn case_003() {
        assert_eq!(
            Solution::find_median_sorted_arrays(vec![1, 2, 3, 4], vec![]),
            2.5
        );
    }

    #[test]
    fn case_004() {
        assert_eq!(
            Solution::find_median_sorted_arrays(vec![1], vec![2, 3, 4]),
            2.5
        );
    }

    #[test]
    fn find_index_001() {
        assert_eq!(Solution::find_index_less_or_equal(&vec![1, 2, 3][..], 0), 0);
    }

    #[test]
    fn find_index_002() {
        assert_eq!(Solution::find_index_less_or_equal(&vec![1, 2, 3][..], 3), 2);
    }

    #[test]
    fn find_index_003() {
        assert_eq!(Solution::find_index_less_or_equal(&vec![1][..], 3), 0);
    }

    #[test]
    fn find_median_value_001() {
        assert_eq!(Solution::find_median_value(&vec![1, 2, 3, 4], 0, 0), 2.5);
    }
}
