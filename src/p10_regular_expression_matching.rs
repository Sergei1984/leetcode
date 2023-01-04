#[allow(dead_code)]

pub struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn is_match(s: String, p: String) -> bool {
        if p == ".*" {
            return true;
        }

        return Self::match_recursive(&s[..], &p[..]);
    }

    fn match_recursive(s: &str, p: &str) -> bool {
        if s.len() == 0 {
            return p.len() == 0;
        }

        let mut p_iter = p.chars();

        if let Some(match_char) = p_iter.next() {
            let mut next_p = &p[1..];
            let mut match_multiple = false;
            let mut stop_char = None;

            if let Some(next) = p_iter.next() {
                if next == '*' {
                    match_multiple = true;
                    next_p = &p[2..];

                    // to handle multiple zero pattern cases like
                    // a*x*y*z*a we need to use second a as stop char
                    loop {
                        stop_char = p_iter.next();
                        if let Some(sc) = stop_char {
                            if sc != match_char {
                                break;
                            }
                        } else {
                            break;
                        }

                        next_p = &next_p[1..];

                        break;
                    }
                }
            }

            let mut idx = 0;
            for c in s.chars() {
                let c_matching = match_char == '.' || c == match_char;

                if !match_multiple {
                    // match single char
                    if c_matching {
                        return Self::match_recursive(&s[1..], next_p);
                    } else {
                        return false;
                    }
                }

                // if current pattern is match-all (.) we test stop char
                // else just wait for the end of the matching

                if match_char == '.' {
                    if let Some(stop_char_value) = stop_char {
                        if c == stop_char_value {
                            return Self::match_recursive(&s[idx..], next_p);
                        }
                    }
                }

                if !c_matching {
                    return Self::match_recursive(&s[idx..], next_p);
                }

                idx = idx + 1;
            }

            return next_p.len() == 0; // end of string means we successfully matched all characters
        } else {
            // end of pattern
            return s.len() == 0;
        }
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn case1() {
        assert_eq!(Solution::is_match("aa".to_string(), "a".to_string()), false);
    }

    #[test]
    fn case2() {
        assert_eq!(Solution::is_match("aa".to_string(), "a*".to_string()), true);
    }

    #[test]
    fn case3() {
        assert_eq!(Solution::is_match("ab".to_string(), ".*".to_string()), true);
    }

    #[test]
    fn case4() {
        assert_eq!(
            Solution::is_match("abc".to_string(), "abc".to_string()),
            true
        );
    }

    #[test]
    fn case5() {
        assert_eq!(
            Solution::is_match("abc".to_string(), "...".to_string()),
            true
        );
    }

    #[test]
    fn case6() {
        assert_eq!(
            Solution::is_match("abc1111111d".to_string(), "a.c.*d".to_string()),
            true
        );
    }

    #[test]
    fn case8() {
        assert_eq!(
            Solution::is_match("abcd".to_string(), "...".to_string()),
            false
        );
    }

    #[test]
    fn case9() {
        assert_eq!(
            Solution::is_match("aaaad".to_string(), "a*d".to_string()),
            true
        );
    }

    #[test]
    fn case10() {
        assert_eq!(
            Solution::is_match("aa".to_string(), "a.*a".to_string()),
            true
        );
    }

    #[test]
    fn case11_match_regex_start() {
        assert_eq!(
            Solution::is_match("abc".to_string(), "abcd".to_string()),
            false
        );
    }
    #[test]
    fn case12() {
        assert_eq!(
            Solution::is_match("mississippi".to_string(), "mis*is*ip*.".to_string()),
            true
        );
    }

    #[test]
    fn case13_non_greedy() {
        let result = Solution::is_match("ab".to_string(), ".*c".to_string());
        assert_eq!(result, false);
    }

    #[test]
    fn case14_regex_longer() {
        let result = Solution::is_match("aaa".to_string(), "aaaa".to_string());
        assert_eq!(result, false);
    }

    #[test]
    fn case15_stop_char_same_as_pattern() {
        let result = Solution::is_match("aaa".to_string(), "a*a".to_string());
        assert_eq!(result, true);
    }

    #[test]
    fn case16_zero_patterns() {
        let result = Solution::is_match("aaa".to_string(), "ab*a*c*a".to_string());
        assert_eq!(result, true);
    }

    #[test]
    fn case17_zero_pattern_at_start() {
        let result = Solution::is_match("aab".to_string(), "c*a*b".to_string());
        assert_eq!(result, true);
    }

    #[test]
    fn case18_multiple_zero_patterns() {
        let result = Solution::is_match("aaa".to_string(), "ab*a*c*d*e*f*a".to_string());
        assert_eq!(result, true);
    }

    #[test]
    fn case19_simplest_zero_pattern() {
        let result = Solution::is_match("aa".to_string(), "a*a".to_string());
        assert_eq!(result, true);
    }

    #[test]
    fn case20_simplest_zero_pattern_interaction() {
        let result = Solution::is_match("aa".to_string(), "a*b*a".to_string());
        assert_eq!(result, true);
    }
}
