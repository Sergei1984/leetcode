use std::collections::HashMap;

#[allow(dead_code)]
pub struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let mut chars = HashMap::<char, usize>::new();
        let mut max_len = 0;

        for (idx, item) in s.chars().enumerate() {
            let mut item_idx: Option<usize> = None;

            {
                let tmp = chars.get(&item);
                if tmp.is_some() {
                    item_idx = Some(tmp.unwrap().clone());
                }
            }

            if let Some(repeat_index) = item_idx {
                max_len = std::cmp::max(max_len, chars.len());

                let mut ri = repeat_index;
                chars.retain(|_k, v| v > &mut ri);
                chars.insert(item, idx);
            } else {
                chars.insert(item, idx);
            }
        }

        max_len = std::cmp::max(max_len, chars.len());

        max_len as i32
    }
}

#[cfg(test)]
mod p1_test {
    use super::Solution;

    #[test]
    fn case1() {
        assert_eq!(
            Solution::length_of_longest_substring("abcabcbb".to_string()),
            3
        );
    }

    #[test]
    fn case2() {
        assert_eq!(
            Solution::length_of_longest_substring("bbbbb".to_string()),
            1
        );
    }

    #[test]
    fn case3() {
        assert_eq!(
            Solution::length_of_longest_substring("pwwkew".to_string()),
            3
        );
    }

    #[test]
    fn case4() {
        assert_eq!(Solution::length_of_longest_substring("aab".to_string()), 2);
    }
}
