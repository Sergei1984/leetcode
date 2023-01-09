pub struct Solution;

impl Solution {
    pub fn is_match(s: String, p: String) -> bool {
        let first_pattern = Self::next_pattern(p.as_str());
        return Self::match_recursive(s.as_str(), first_pattern);
    }

    fn match_recursive<'a>(s: &str, p: Option<Pattern<'a>>) -> bool {
        if let Some(pattern) = p {
            let next_char = s.chars().next();

            if pattern.allow_multiple {
                let maybe_next_pattern = Self::next_pattern(pattern.next_regex);

                if let Some(next) = next_char {
                    if pattern.match_char == '.' || next == pattern.match_char {
                        if pattern.match_char == '.'
                            || maybe_next_pattern
                                .as_ref()
                                .map(|n| {
                                    n.allow_multiple
                                        || n.match_char == '.'
                                        || n.match_char == pattern.match_char
                                })
                                .unwrap_or(false)
                        {
                            let is_matching_if_end_of_greed =
                                Self::match_recursive(&s, maybe_next_pattern);

                            if is_matching_if_end_of_greed {
                                return true;
                            }
                        }

                        return Self::match_recursive(&s[1..], Some(pattern));
                    } else {
                        return Self::match_recursive(&s, maybe_next_pattern);
                    }
                } else {
                    return Self::match_recursive(&s, maybe_next_pattern);
                }
            } else {
                if let Some(next) = next_char {
                    if pattern.match_char == '.' || next == pattern.match_char {
                        return Self::match_recursive(
                            &s[1..],
                            Self::next_pattern(pattern.next_regex),
                        );
                    } else {
                        return false;
                    }
                } else {
                    return false;
                }
            }
        } else {
            return s.len() == 0;
        }
    }

    #[inline]
    fn next_pattern<'a>(regex: &'a str) -> Option<Pattern<'a>> {
        let mut next_index = 1;
        let mut chars = regex.chars();

        let next_char = chars.next();
        if next_char.is_none() {
            return None;
        }

        let match_char = next_char.unwrap();

        let allow_multiple = chars.next().unwrap_or_default() == '*';
        if allow_multiple {
            next_index = next_index + 1;
        }

        Some(Pattern {
            match_char,
            allow_multiple,
            next_regex: &regex[next_index..],
        })
    }
}

#[derive(Debug)]
pub struct Pattern<'a> {
    pub match_char: char,
    pub allow_multiple: bool,
    pub next_regex: &'a str,
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn case001() {
        assert_eq!(Solution::is_match("aa".to_string(), "a".to_string()), false);
    }

    #[test]
    fn case002() {
        assert_eq!(Solution::is_match("aa".to_string(), "aa".to_string()), true);
    }

    #[test]
    fn case003() {
        assert_eq!(Solution::is_match("ab".to_string(), "ab".to_string()), true);
    }

    #[test]
    fn case004() {
        assert_eq!(Solution::is_match("".to_string(), "".to_string()), true);
    }

    #[test]
    fn case005() {
        assert_eq!(Solution::is_match("a".to_string(), "".to_string()), false);
    }

    #[test]
    fn case006() {
        assert_eq!(Solution::is_match("".to_string(), "a".to_string()), false);
    }

    #[test]
    fn case007() {
        assert_eq!(Solution::is_match("b".to_string(), "a".to_string()), false);
    }

    #[test]
    fn case008() {
        assert_eq!(
            Solution::is_match("ab".to_string(), "ac".to_string()),
            false
        );
    }

    #[test]
    fn case009() {
        assert_eq!(Solution::is_match("ab".to_string(), "a.".to_string()), true);
    }

    #[test]
    fn case010() {
        assert_eq!(
            Solution::is_match("abcd".to_string(), "....".to_string()),
            true
        );
    }

    #[test]
    fn case011() {
        assert_eq!(
            Solution::is_match("abcd".to_string(), "...".to_string()),
            false
        );
    }

    #[test]
    fn case012_match_multiple() {
        assert_eq!(
            Solution::is_match("aaad".to_string(), "a*d".to_string()),
            true
        );
    }

    #[test]
    fn case013_match_multiple_to_one() {
        assert_eq!(
            Solution::is_match("ad".to_string(), "a*d".to_string()),
            true
        );
    }

    #[test]
    fn case014_match_mutiple_to_zero() {
        assert_eq!(Solution::is_match("d".to_string(), "a*d".to_string()), true);
    }

    #[test]
    fn case015_match_mutiple_at_the_end_of_the_string() {
        assert_eq!(
            Solution::is_match("ab".to_string(), "abc*".to_string()),
            true
        );
    }

    #[test]
    fn case016_match_several_mutiple_at_the_end_of_the_string() {
        assert_eq!(
            Solution::is_match("ab".to_string(), "abc*d*e*".to_string()),
            true
        );
    }

    #[test]
    fn case017_match_several_at_the_start_of_string() {
        assert_eq!(
            Solution::is_match("bc".to_string(), "a*bc".to_string()),
            true
        );
    }

    #[test]
    fn case018_match_multiple_several_at_the_start_of_string() {
        assert_eq!(
            Solution::is_match("de".to_string(), "a*b*c*de".to_string()),
            true
        );
    }

    #[test]
    fn case019_match_all() {
        assert_eq!(Solution::is_match("ab".to_string(), ".*".to_string()), true);
    }

    #[test]
    fn case020() {
        assert_eq!(
            Solution::is_match("abc1111111d".to_string(), "a.c.*d".to_string()),
            true
        );
    }

    #[test]
    fn case021() {
        assert_eq!(
            Solution::is_match("aa".to_string(), "a.*a".to_string()),
            true
        );
    }

    #[test]
    fn case022_leetcode_test_case() {
        assert_eq!(
            Solution::is_match("mississippi".to_string(), "mis*is*ip*.".to_string()),
            true
        );
    }

    #[test]
    fn case023_non_greedy() {
        let result = Solution::is_match("ab".to_string(), ".*c".to_string());
        assert_eq!(result, false);
    }

    #[test]
    fn case024_regex_longer() {
        let result = Solution::is_match("aaa".to_string(), "aaaa".to_string());
        assert_eq!(result, false);
    }

    #[test]
    fn case025_stop_char_same_as_pattern() {
        let result = Solution::is_match("aaa".to_string(), "a*a".to_string());
        assert_eq!(result, true);
    }

    #[test]
    fn case026_multiple_zero_patterns() {
        let result = Solution::is_match("aaa".to_string(), "ab*a*c*d*e*f*a".to_string());
        assert_eq!(result, true);
    }

    #[test]
    fn case027_simplest_zero_pattern() {
        let result = Solution::is_match("aa".to_string(), "a*a".to_string());
        assert_eq!(result, true);
    }

    #[test]
    fn case028_simplest_zero_pattern_interaction() {
        let result = Solution::is_match("aa".to_string(), "a*b*a".to_string());
        assert_eq!(result, true);
    }

    #[test]
    fn case029_fucked_up_lookup() {
        let result = Solution::is_match("aaacacaab".to_string(), ".*ab".to_string());
        assert_eq!(result, true);
    }

    #[test]
    fn case030_zero_or_more_at_the_end() {
        let result = Solution::is_match("a".to_string(), "ab*".to_string());
        assert_eq!(result, true);
    }
}
