#![allow(unused)]

pub struct Solution;

impl Solution {
    pub fn sort(input: &mut Vec<i32>) {}

    // [1, 100, 200, 20, 120, 250]
    // [200, 250, 300, 1, 2, 3]

    // [1, 5, 19, 17, 18, 20]
    pub fn merge_sorted_parts_into_new(first: &[i32], second: &[i32]) -> Vec<i32> {
        let mut result = Vec::with_capacity(first.len() + second.len());

        let mut h1 = 0;
        let mut h2 = 0;

        loop {
            let o1 = first.get(h1);
            let o2 = second.get(h2);

            if let Some(e1) = o1 {
                if let Some(e2) = o2 {
                    if (*e1 < *e2) {
                        result.push(*e1);
                        h1 += 1;
                    } else {
                        result.push(*e2);
                        h2 += 1;
                    }
                } else {
                    result.push(*e1);
                    h1 += 1;
                }
            } else {
                if let Some(e2) = o2 {
                    result.push(*e2);
                    h2 += 1;
                } else {
                    break;
                }
            }
        }

        result
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    // #[test]
    // fn case_001() {
    //     let mut input = vec![10, 2, 33, 50, 1];

    //     Solution::sort(&mut input);

    //     assert_eq!(input, vec![1, 2, 10, 33, 50]);
    // }

    #[test]
    fn merge_outplace_001() {
        let result = Solution::merge_sorted_parts_into_new(
            &(vec![1, 2, 10, 11])[..],
            &(vec![3, 4, 5, 20])[..],
        );

        assert_eq!(vec![1, 2, 3, 4, 5, 10, 11, 20], result);
    }

    #[test]
    fn merge_outplace_002() {
        let result = Solution::merge_sorted_parts_into_new(&(vec![11])[..], &(vec![20])[..]);

        assert_eq!(vec![11, 20], result);
    }
}
