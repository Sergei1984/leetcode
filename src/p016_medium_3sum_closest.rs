#![allow(unused)]

pub struct Solution;

impl Solution {
    pub fn three_sum_closest(nums: Vec<i32>, target: i32) -> i32 {
        if nums.len() == 0 {
            return 0;
        }

        let mut n = nums.clone();

        n.sort();

        let mut global_min_diff = i32::MAX;
        let mut global_closest_sum = i32::MAX;

        let mut min = 0;
        let mut max = n.len() - 1;

        loop {
            if max - min <= 1 {
                break;
            }

            let reminder = n[min] + n[max];

            let mut closest = i32::MAX;
            let mut diff = i32::MAX;

            let inner = &n[min + 1..max];
            let idx = inner.partition_point(|x| *x < reminder);

            let start = if idx > 0 { idx - 1 } else { idx };

            for num in &inner[start..(idx + 2).min(inner.len())] {
                let sum = reminder + num;

                let current_diff = (sum - target).abs();

                if current_diff < diff {
                    diff = current_diff;
                    closest = sum;

                    if diff == 0 {
                        return sum;
                    }
                }
            }

            if global_min_diff > diff {
                global_min_diff = diff;
                global_closest_sum = closest;
            }

            if closest > target {
                max -= 1;
            }
            if closest < target {
                min += 1;
            }
        }

        global_closest_sum
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
}
