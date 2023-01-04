#[allow(dead_code)]

pub struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn my_atoi(s: String) -> i32 {
        let mut is_negative = false;
        let mut result: i32 = 0;
        let mut is_digits_started = false;

        for c in s.chars() {
            if let Some(digit) = c.to_digit(10) {
                let (r, ofl) = result.overflowing_mul(10);
                if ofl {
                    return if is_negative { i32::MIN } else { i32::MAX };
                }

                let (r2, ofl2) = r.overflowing_add(digit as i32);

                if ofl2 {
                    return if is_negative { i32::MIN } else { i32::MAX };
                }

                result = r2;
                is_digits_started = true;
            } else {
                if is_digits_started {
                    break;
                } else {
                    match c {
                        '-' => {
                            is_digits_started = true;
                            is_negative = true;
                        }
                        ' ' => {
                            continue;
                        }
                        '+' => {
                            is_digits_started = true;
                            continue;
                        }
                        _ => {
                            return 0;
                        }
                    }
                }
            }
        }

        result * if is_negative { -1 } else { 1 }
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn case1() {
        assert_eq!(Solution::my_atoi("42".to_string()), 42);
    }

    #[test]
    fn case2() {
        assert_eq!(Solution::my_atoi("-42".to_string()), -42);
    }

    #[test]
    fn case3() {
        assert_eq!(Solution::my_atoi("+42".to_string()), 42);
    }
    #[test]
    fn case4() {
        assert_eq!(Solution::my_atoi("+0042".to_string()), 42);
    }

    #[test]
    fn case5() {
        assert_eq!(Solution::my_atoi("    -42".to_string()), -42);
    }

    #[test]
    fn case6() {
        assert_eq!(
            Solution::my_atoi("    -42 and some shit after".to_string()),
            -42
        );
    }

    #[test]
    fn case7() {
        assert_eq!(Solution::my_atoi("words and 987".to_string()), 0);
    }

    #[test]
    fn case8() {
        assert_eq!(Solution::my_atoi("-91283472332".to_string()), -2147483648);
    }

    #[test]
    fn case9() {
        assert_eq!(Solution::my_atoi("+-12".to_string()), 0);
    }
}
