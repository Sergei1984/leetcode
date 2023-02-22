#![allow(unused)]

pub struct Solution;

impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        let digits: Vec<char> = x.to_string().chars().collect();

        if digits.len() < 1 {
            return true;
        }

        let left;
        let right;

        if digits.len() % 2 == 0 {
            right = digits.len() / 2;
            left = right - 1;
        } else {
            right = digits.len() / 2 + 1;
            left = right - 2;
        }

        for i in 0..left + 1 {
            if digits[left - i] != digits[right + i] {
                return false;
            }
        }

        true
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    pub fn case_001() {
        assert_eq!(true, Solution::is_palindrome(121));
    }

    #[test]
    pub fn case_002() {
        assert_eq!(true, Solution::is_palindrome(22));
    }

    #[test]
    pub fn case_003() {
        assert_eq!(true, Solution::is_palindrome(2332));
    }

    #[test]
    pub fn case_004() {
        assert_eq!(true, Solution::is_palindrome(23532));
    }

    #[test]
    pub fn case_005() {
        assert_eq!(false, Solution::is_palindrome(10));
    }
}
