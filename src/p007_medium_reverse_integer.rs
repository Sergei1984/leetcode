#[allow(dead_code)]

pub struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn reverse(x: i32) -> i32 {
        let iter = ReverseDigitIterator::new(x);

        let mut result: i32 = 0;
        let mut position: i32 = 1;

        for d in iter.skip_while(|d| *d == 0) {
            let (next_res, ofl) = result.overflowing_mul(position);
            if ofl {
                return 0;
            }

            let (next_res2, ofl) = next_res.overflowing_add(d);

            if ofl {
                return 0;
            }

            result = next_res2;
            if position == 1 {
                position = 10;
            }
        }

        result
    }
}

struct ReverseDigitIterator {
    number: i32,
    pos: i32,
    last_reminder: i32,
    completed: bool,
    overflowing: bool,
}

impl ReverseDigitIterator {
    pub fn new(number: i32) -> Self {
        ReverseDigitIterator {
            number: number,
            pos: 10,
            last_reminder: 0,
            completed: false,
            overflowing: false,
        }
    }
}

impl Iterator for ReverseDigitIterator {
    type Item = i32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.completed {
            return None;
        }

        if self.overflowing {
            self.completed = true;
            if self.number < 0 {
                return Some(if self.number <= -2000000000 { -2 } else { -1 });
            } else {
                return Some(if self.number < 2000000000 { 1 } else { 2 });
            }
        }

        let rem = self.number % self.pos;
        if rem == self.number {
            self.completed = true;
        }

        let next_val = (rem - self.last_reminder) / (self.pos / 10);

        self.last_reminder = rem;

        let (p, ofl) = self.pos.overflowing_mul(10);

        if ofl {
            self.overflowing = true;
        } else {
            self.pos = p;
        }

        Some(next_val)
    }
}

#[cfg(test)]
mod test {
    use crate::p007_medium_reverse_integer::ReverseDigitIterator;

    use super::Solution;

    #[test]
    fn case001() {
        assert_eq!(Solution::reverse(123), 321);
    }

    #[test]
    fn case002() {
        assert_eq!(Solution::reverse(-123), -321);
    }

    #[test]
    fn case003() {
        assert_eq!(Solution::reverse(120), 21);
    }

    #[test]
    fn case004() {
        assert_eq!(Solution::reverse(0), 0);
    }

    #[test]
    fn case005() {
        assert_eq!(Solution::reverse(1), 1);
    }

    #[test]
    fn case006() {
        assert_eq!(Solution::reverse(-1), -1);
    }

    #[test]
    fn case007() {
        assert_eq!(Solution::reverse(-2147483412), -2143847412);
    }

    #[test]
    fn case009() {
        assert_eq!(Solution::reverse(1463847412), 2147483641);
    }

    #[test]
    fn case0010() {
        let i = ReverseDigitIterator::new(-2147483412);
        for d in i {
            println!("{}", d);
        }
    }

    #[test]
    fn case0011() {
        let i = ReverseDigitIterator::new(1463847412);
        for d in i {
            println!("{}", d);
        }
    }

    #[test]
    fn case012() {
        assert_eq!(Solution::reverse(-1463847412), -2147483641);
    }

    #[test]
    fn case0013() {
        let i = ReverseDigitIterator::new(-1463847412);
        for d in i {
            println!("{}", d);
        }
    }
}
