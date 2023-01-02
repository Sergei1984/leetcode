use std::collections::HashMap;

#[allow(dead_code)]
pub struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut map = HashMap::<i32, usize>::new();

        for i in 0..nums.len() {
            let value = target - nums[i];

            let item = map.get(&value);
            if let Some(index) = item {
                return vec![*index as i32, i as i32];
            }

            map.insert(nums[i], i);
        }

        vec![]
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
