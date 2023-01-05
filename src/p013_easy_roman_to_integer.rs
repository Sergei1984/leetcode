#![allow(unused)]

pub struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn roman_to_int(s: String) -> i32 {
        let mut result = 0;
        let roman = ['I', 'V', 'X', 'L', 'C', 'D', 'M'];
        let ints = [1, 5, 10, 50, 100, 500, 1000];

        let mut prev_index: Option<usize> = None;

        for sym in s.chars() {
            let index = roman.iter().position(|i| *i == sym).unwrap();

            let i = ints[index];
            result = result + i;

            if let Some(prev) = prev_index {
                if (prev == 0 || prev == 2 || prev == 4) && index > prev && index <= prev + 2 {
                    result = result - 2 * ints[prev];
                }
            }

            prev_index = Some(index);
        }

        return result;
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn case1() {
        assert_eq!(Solution::roman_to_int(String::from("III")), 3);
    }

    #[test]
    fn case2() {
        assert_eq!(Solution::roman_to_int("LVIII".to_string()), 58);
    }

    #[test]
    fn case3() {
        assert_eq!(Solution::roman_to_int("MCMXCIV".to_string()), 1994);
    }
}
