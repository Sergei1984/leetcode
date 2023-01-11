#![allow(unused)]

pub struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn int_to_roman(num: i32) -> String {
        let roman = ['I', 'V', 'X', 'L', 'C', 'D', 'M'];
        let ints = [1, 5, 10, 50, 100, 500, 1000];
        let mut roman: Vec<u8> = vec![];

        for (num, base) in IntBaseIterator::new(num) {}

        String::from_utf8(roman).unwrap()
    }
}

pub struct BaseConversionInfo {
    pub base: i32, // ie 10, 100, 1000 etc
    pub num_pairs: Vec<(byte, )
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
}
