#![allow(unused)]

pub struct Solution;

impl Solution {
    pub fn three_sum_closest(nums: Vec<i32>, target: i32) -> i32 {
        if nums.len() == 0 {
            return 0;
        }

        let mut n = nums.clone();

        n.sort();

        let mut diff = i32::MAX;
        let mut closest_sum = i32::MAX;

        for min in 0..n.len() - 1 {
            for max in (min + 1..n.len()).rev() {
                let reminder = n[min] + n[max];

                let inner = &n[min + 1..max];
                let start = inner
                    .partition_point(|x| *x <= reminder)
                    .min(if inner.len() == 0 { 0 } else { inner.len() - 1 });

                let mut end = if inner.len() == 0 { 0 } else { inner.len() - 1 };
                for i in start..inner.len() {
                    if inner[i] > reminder {
                        end = i;
                        break;
                    }
                }

                for num in &inner[start..end] {
                    let sum = reminder + *num;

                    let current_diff = (sum - target).abs();

                    if current_diff < diff {
                        diff = current_diff;
                        closest_sum = sum;

                        if diff == 0 {
                            return sum;
                        }
                    }
                }
            }
        }

        closest_sum
    }

    // [-5, -2, 0, 5, 7], 3
    //     vvvv
    // [-8, -5, 3, 2, 4]
    //     vvvv
    // (val=5, idx=3)
    pub fn find_closest(n: &[i32], target: i32) -> Option<(i32, usize)> {
        // let mapped: Vec<i32> = n.iter().map(|x| *x - target).collect();

        if n.len() == 0 {
            return None;
        }

        if n.len() == 1 {
            return Some((n[0], 0));
        }

        let index = n.partition_point(|x| *x - target < 0);

        // all items are less
        if index == n.len() {
            return Some((n[n.len() - 1], n.len() - 1));
        }

        if index == 0 {
            return Some((n[0], 0));
        }

        let d = (n[index] - target).abs();

        let d_next = n
            .get(index + 1)
            .map(|v| (*v - target).abs())
            .unwrap_or(i32::MAX);

        let d_prev = n
            .get(index - 1)
            .map(|v| (*v - target).abs())
            .unwrap_or(i32::MAX);

        if d < d_next {
            if d < d_prev {
                return Some((n[index], index));
            } else {
                return Some((n[index - 1], index - 1));
            }
        } else {
            return Some((n[index + 1], index + 1));
        }
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    pub fn case_001() {
        assert_eq!(Solution::three_sum_closest(vec![-1, 2, 1, -4], 1), 2);
    }

    #[test]
    pub fn case_002() {
        assert_eq!(Solution::three_sum_closest(vec![0, 0, 0], 1), 0);
    }

    #[test]
    pub fn case_003() {
        assert_eq!(Solution::three_sum_closest(vec![1, 1, 1, 0], 100), 3);
    }

    #[test]
    pub fn case_004() {
        assert_eq!(
            Solution::three_sum_closest(vec![4, 0, 5, -5, 3, 3, 0, -4, -5], -2),
            -2
        );
    }

    #[test]
    pub fn find_closest_001() {
        assert_eq!(
            Solution::find_closest(&vec![-3, -2, 0, 1, 10], 2),
            Some((1, 3))
        );
    }

    #[test]
    pub fn find_closest_002() {
        assert_eq!(
            Solution::find_closest(&vec![-3, -2, 0, 1, 10], -3),
            Some((-3, 0))
        );
    }

    #[test]
    pub fn find_closest_003() {
        assert_eq!(
            Solution::find_closest(&vec![-3, -2, 0, 1, 10], -100),
            Some((-3, 0))
        );
    }

    #[test]
    pub fn find_closest_004() {
        assert_eq!(
            Solution::find_closest(&vec![-3, -2, 0, 1, 10], 100),
            Some((10, 4))
        );
    }

    #[test]
    pub fn find_closest_005() {
        assert_eq!(
            Solution::find_closest(&vec![-3, -2, 0, 1, 10], 11),
            Some((10, 4))
        );
    }
}
