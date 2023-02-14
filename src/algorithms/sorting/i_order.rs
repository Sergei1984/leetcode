#![allow(unused)]

pub struct Solution;

impl Solution {
    pub fn sort(input: &mut Vec<i32>) {}

    // Distribute slice elements around n-th element, so left side would contain smaller elements and right side would contain larger
    pub fn partition<T: PartialOrd>(input: &mut [T], n: usize) -> usize {
        input.swap(n, input.len() - 1);

        let last_idx = input.len() - 1;

        let mut lower_larger_idx = 0;

        for i in 0..last_idx {
            if input[i].lt(&input[last_idx]) {
                input.swap(lower_larger_idx, i);
                lower_larger_idx += 1;
            }
        }

        input.swap(last_idx, lower_larger_idx);

        lower_larger_idx
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn partition_001() {
        let mut arr = vec![21, 1, 5, 12, 77, 9, 22];
        let pivot_idx = 6;
        let pivot_val = arr[pivot_idx];

        Solution::partition(&mut arr, pivot_idx);

        println!("Pivot is {}", pivot_val);
        println!("{:?}", arr);
    }
}
