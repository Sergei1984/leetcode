use std::collections::{BinaryHeap, HashMap, VecDeque};

#[allow(dead_code)]
pub struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn longest_palindrome(s: String) -> String {
        if s.len() == 0 {
            return "".to_string();
        }

        if s.len() == 1 {
            return s;
        }

        let mut symbols = HashMap::<char, VecDeque<usize>>::new();

        for (i, s) in s.chars().enumerate() {
            let entry = symbols.get_mut(&s);
            if let Some(deq) = entry {
                deq.push_back(i);
            } else {
                let mut value = VecDeque::<usize>::new();
                value.push_back(i);
                symbols.insert(s, value);
            }
        }

        let mut indexes_heap = BinaryHeap::new();
        for (_, idx) in symbols.iter() {
            for i in 0..idx.len() - 1 {
                for y in i + 1..idx.len() {
                    let pair = IndexPair {
                        min: *idx.get(i).unwrap(),
                        max: *idx.get(y).unwrap(),
                    };
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

        "".to_string()
    }

    #[inline]
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

#[derive(Eq, PartialEq)]
struct IndexPair {
    pub min: usize,
    pub max: usize,
}

impl PartialOrd for IndexPair {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let my_distance = self.max - self.min;
        let other_distance = self.max - self.min;

        if my_distance != other_distance {
            return Some(my_distance.cmp(&other_distance));
        }

        return Some(self.min.cmp(&other.min));
    }
}

impl Ord for IndexPair {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let my_distance = self.max - self.min;
        let other_distance = self.max - self.min;

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
}
