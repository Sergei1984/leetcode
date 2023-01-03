#![allow(unused)]

use std::collections::BinaryHeap;

pub struct Solution;
impl Solution {
    pub fn longest_palindrome(s: String) -> String {
        if s.len() == 0 {
            return "".to_string();
        }

        if s.len() == 1 {
            return s;
        }

        let b = s.as_bytes();

        if b.len() == 2 {
            if b[0] == b[1] {
                return s;
            } else {
                return s[0..1].to_string();
            }
        }

        let mut indexes_heap = BinaryHeap::new();

        for i in 0..b.len() - 1 {
            for k in i + 1..b.len() {
                if b[i] == b[k] {
                    let pair = IndexPair { min: i, max: k };

                    indexes_heap.push(pair);
                }
            }
        }

        loop {
            let next = indexes_heap.pop();

            if let Some(pair) = next {
                let substr = &s[pair.min..pair.max + 1];
                if Self::is_palindrome(substr) {
                    return substr.to_string();
                }
            } else {
                break;
            }
        }

        s[0..1].to_string()
    }

    #[inline]
    #[allow(dead_code)]
    pub fn is_palindrome(s: &str) -> bool {
        let b = s.as_bytes();
        for i in 0..b.len() / 2 {
            if b[i] != b[b.len() - i - 1] {
                return false;
            }
        }
        true
    }
}

#[derive(Eq, PartialEq, Debug)]
struct IndexPair {
    pub min: usize,
    pub max: usize,
}

impl PartialOrd for IndexPair {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for IndexPair {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let my_distance = self.max - self.min;
        let other_distance = other.max - other.min;

        if my_distance != other_distance {
            return my_distance.cmp(&other_distance);
        }

        return self.min.cmp(&other.min);
    }
}
#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn case1() {
        assert_eq!(
            Solution::longest_palindrome("babad".to_string()),
            "aba".to_string()
        );
    }

    #[test]
    fn case2() {
        assert_eq!(
            Solution::longest_palindrome("cbbd".to_string()),
            "bb".to_string()
        );
    }

    #[test]
    fn case3() {
        assert_eq!(
            Solution::longest_palindrome("c".to_string()),
            "c".to_string()
        );
    }

    #[test]
    fn case4() {
        assert_eq!(Solution::is_palindrome("c"), true);
    }

    #[test]
    fn case5() {
        assert_eq!(Solution::is_palindrome("cc"), true);
    }

    #[test]
    fn case6() {
        assert_eq!(Solution::is_palindrome("acca"), true);
    }

    #[test]
    fn case7() {
        assert_eq!(Solution::is_palindrome("accaa"), false);
    }

    #[test]
    fn case8() {
        assert_eq!(
            Solution::longest_palindrome("abcccss".to_string()),
            "ccc".to_string()
        );
    }

    #[test]
    fn case9() {
        assert_eq!(
            Solution::longest_palindrome("abccsv".to_string()),
            "cc".to_string()
        );
    }

    #[test]
    fn case10() {
        assert_eq!(
            Solution::longest_palindrome("ac".to_string()),
            "a".to_string()
        );
    }

    #[test]
    fn case11() {
        assert_eq!(
            Solution::longest_palindrome("aacabdkacaa".to_string()),
            "aca".to_string()
        );
    }
}
