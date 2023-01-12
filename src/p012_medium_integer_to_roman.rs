#![allow(unused)]

use std::{collections::HashMap, iter::Map};

pub struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn int_to_roman(num: i32) -> String {
        let rules = Self::rules();
        let mut result = String::new();

        let mut digits_with_base: Vec<(i32, i32)> = IntBaseIterator::new(num).collect();
        digits_with_base.reverse();

        for (num, base) in digits_with_base.iter() {
            if *num == 0 {
                continue;
            }

            let r = Self::int_to_roman_base(*num, *base, &rules);
            result.push_str(r.as_str());
        }

        result
    }

    pub fn rules() -> HashMap<i32, BaseConversionRule> {
        HashMap::from([
            (
                1,
                BaseConversionRule {
                    base: 1,
                    num_pairs: vec![(5, "V"), (1, "I")],
                    exceptions: HashMap::from([(4, "IV"), (9, "IX")]),
                },
            ),
            (
                10,
                BaseConversionRule {
                    base: 10,
                    num_pairs: vec![(50, "L"), (10, "X")],
                    exceptions: HashMap::from([(40, "XL"), (90, "XC")]),
                },
            ),
            (
                100,
                BaseConversionRule {
                    base: 100,
                    num_pairs: vec![(500, "D"), (100, "C")],
                    exceptions: HashMap::from([(400, "CD"), (900, "CM")]),
                },
            ),
            (
                1000,
                BaseConversionRule {
                    base: 1000,
                    num_pairs: vec![(1000, "M")],
                    exceptions: HashMap::new(),
                },
            ),
        ])
    }

    pub fn int_to_roman_base(
        num: i32,
        base: i32,
        rules: &HashMap<i32, BaseConversionRule>,
    ) -> String {
        let rule = rules.get(&base).unwrap();

        let mut result = String::new();
        let mut digits = rule.num_pairs.iter();

        let mut reminder = num;
        let mut digit = digits.next().unwrap();

        loop {
            let exception = rule.exceptions.get(&reminder);
            if let Some(exc) = exception {
                result.push_str(*exc);
                break;
            }

            if reminder >= digit.0 {
                result.push_str(digit.1);
                reminder = reminder - digit.0;

                if reminder <= 0 {
                    break;
                }
            } else {
                digit = digits.next().unwrap();
            }
        }

        result
    }
}

pub struct BaseConversionRule {
    pub base: i32, // ie 10, 100, 1000 etc
    pub num_pairs: Vec<(i32, &'static str)>,
    pub exceptions: HashMap<i32, &'static str>,
}

pub struct IntBaseIterator {
    number: i32,
    last_reminder: i32,
    current_base: i32,
    is_completed: bool,
}

impl IntBaseIterator {
    pub fn new(number: i32) -> Self {
        IntBaseIterator {
            number,
            last_reminder: 0,
            current_base: 10,
            is_completed: false,
        }
    }
}

impl Iterator for IntBaseIterator {
    type Item = (i32, i32);

    fn next(&mut self) -> Option<Self::Item> {
        if self.is_completed {
            return None;
        }

        let current_reminder = self.number % self.current_base;
        if current_reminder == self.number {
            self.is_completed = true;
        }

        let result = current_reminder - self.last_reminder;
        let base = self.current_base / 10;

        self.last_reminder = current_reminder;
        self.current_base = self.current_base * 10;

        Some((result, base))
    }
}

#[cfg(test)]
mod test {
    use super::{IntBaseIterator, Solution};

    #[test]
    fn number_positions_001() {
        let mut positions: Vec<(i32, i32)> = IntBaseIterator::new(20).collect();
        assert_eq!([(0, 1), (20, 10)].to_vec(), positions);
    }

    #[test]
    fn number_positions_002() {
        let mut positions: Vec<(i32, i32)> = IntBaseIterator::new(1).collect();
        assert_eq!([(1, 1)].to_vec(), positions);
    }

    #[test]
    fn number_positions_003() {
        let mut positions: Vec<(i32, i32)> = IntBaseIterator::new(2903).collect();
        assert_eq!(
            [(3, 1), (0, 10), (900, 100), (2000, 1000)].to_vec(),
            positions
        );
    }

    #[test]
    fn int_to_roman_base_001() {
        let rules = Solution::rules();
        assert_eq!("XXX", Solution::int_to_roman_base(30, 10, &rules));
    }

    #[test]
    fn int_to_roman_base_002() {
        let rules = Solution::rules();
        assert_eq!("III", Solution::int_to_roman_base(3, 1, &rules));
    }

    #[test]
    fn int_to_roman_base_003() {
        let rules = Solution::rules();
        assert_eq!("IV", Solution::int_to_roman_base(4, 1, &rules));
    }

    #[test]
    fn int_to_roman_base_004() {
        let rules = Solution::rules();
        assert_eq!("VIII", Solution::int_to_roman_base(8, 1, &rules));
    }

    #[test]
    fn int_to_roman_base_005() {
        let rules = Solution::rules();
        assert_eq!("LX", Solution::int_to_roman_base(60, 10, &rules));
    }

    #[test]
    fn int_to_roman_base_006() {
        let rules = Solution::rules();
        assert_eq!("XC", Solution::int_to_roman_base(90, 10, &rules));
    }

    #[test]
    fn int_to_roman_base_007() {
        let rules = Solution::rules();
        assert_eq!("MMMMMM", Solution::int_to_roman_base(6000, 1000, &rules));
    }

    #[test]
    fn case_001() {
        assert_eq!(Solution::int_to_roman(3), String::from("III"));
    }

    #[test]
    fn case_002() {
        assert_eq!(Solution::int_to_roman(58), "LVIII".to_string());
    }

    #[test]
    fn case_003() {
        assert_eq!(Solution::int_to_roman(1994), "MCMXCIV".to_string());
    }

    #[test]
    fn case_004() {
        assert_eq!(Solution::int_to_roman(10), "X".to_string());
    }
}
