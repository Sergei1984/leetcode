#![allow(unused)]

pub struct Solution;

impl Solution {
    pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
        let mut t1 = (m - 1) as usize;
        let mut t2 = (n - 1) as usize;

        let mut current = (m + n - 1) as usize;

        loop {
            let o1 = nums1.get(t1);
            let o2 = nums2.get(t2);

            if let Some(v1) = o1 {
                if let Some(v2) = o2 {
                    if *v1 > *v2 {
                        nums1[current] = *v1;
                        current -= 1;
                        t1 -= 1;
                    } else {
                        nums1[current] = *v2;
                        current -= 1;
                        t2 -= 1;
                    }
                } else {
                    nums1[current] = *v1;
                    current -= 1;
                    t1 -= 1;
                }
            } else {
                if let Some(v2) = o2 {
                    nums1[current] = *v2;
                    current -= 1;
                    t2 -= 1;
                } else {
                    break;
                }
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::Solution;
}
