#[allow(dead_code)]

pub struct Solution;

impl Solution {
    // Main trick: if we have multiple match pattern `a*` we should try to finish it each matching step
    // Otherwise we never know how end of pattern would match
    pub fn is_match(s: String, p: String) -> bool {
        if p == ".*" {
            return true;
        }

        return Self::match_recursive(&s[..], &p[..]);
    }

    fn match_recursive(s: &str, p: &str) -> bool {
        let curr_p = Self::next_pattern(p);
        if let Some(pattern) = curr_p {
            if s.len() == 0 && !pattern.allow_multiple {
                return false;
            }

            let mut idx = 0;
            for c in s.chars() {
                // The simplest case
                if !pattern.allow_multiple {
                    if pattern.match_char == c || pattern.match_char == '.' {
                        return Self::match_recursive(&s[idx + 1..], pattern.next_regex);
                    } else {
                        return false;
                    }
                }

                // Handle zero or any pattern
                let match_current = pattern.match_char == c || pattern.match_char == '.';
                if !match_current {
                    return Self::match_recursive(&s[idx..], pattern.next_regex);
                }

                if pattern.next_regex.len() > 0 {
                    let next_fully_match = Self::match_recursive(&s[idx..], pattern.next_regex);
                    if next_fully_match {
                        return true;
                    }
                }

                idx = idx + 1;
            }

            return Self::match_recursive(&s[idx..], pattern.next_regex);
        } else {
            return s.len() == 0;
        }
    }

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
        assert_eq!(Solution::is_match("aa".to_string(), "a*".to_string()), true);
    }

    #[test]
    fn case003() {
        assert_eq!(Solution::is_match("ab".to_string(), ".*".to_string()), true);
    }

    #[test]
    fn case004() {
        assert_eq!(
            Solution::is_match("abc".to_string(), "abc".to_string()),
            true
        );
    }

    #[test]
    fn case005() {
        assert_eq!(
            Solution::is_match("abc".to_string(), "...".to_string()),
            true
        );
    }

    #[test]
    fn case006() {
        assert_eq!(
            Solution::is_match("abc1111111d".to_string(), "a.c.*d".to_string()),
            true
        );
    }

    #[test]
    fn case008() {
        assert_eq!(
            Solution::is_match("abcd".to_string(), "...".to_string()),
            false
        );
    }

    #[test]
    fn case009() {
        assert_eq!(
            Solution::is_match("aaaad".to_string(), "a*d".to_string()),
            true
        );
    }

    #[test]
    fn case010() {
        assert_eq!(
            Solution::is_match("aa".to_string(), "a.*a".to_string()),
            true
        );
    }

    #[test]
    fn case011_match_regex_start() {
        assert_eq!(
            Solution::is_match("abc".to_string(), "abcd".to_string()),
            false
        );
    }
    #[test]
    fn case012() {
        assert_eq!(
            Solution::is_match("mississippi".to_string(), "mis*is*ip*.".to_string()),
            true
        );
    }

    #[test]
    fn case013_non_greedy() {
        let result = Solution::is_match("ab".to_string(), ".*c".to_string());
        assert_eq!(result, false);
    }

    #[test]
    fn case014_regex_longer() {
        let result = Solution::is_match("aaa".to_string(), "aaaa".to_string());
        assert_eq!(result, false);
    }

    #[test]
    fn case015_stop_char_same_as_pattern() {
        let result = Solution::is_match("aaa".to_string(), "a*a".to_string());
        assert_eq!(result, true);
    }

    #[test]
    fn case016_zero_patterns() {
        let result = Solution::is_match("aaa".to_string(), "ab*a*c*a".to_string());
        assert_eq!(result, true);
    }

    #[test]
    fn case017_zero_pattern_at_start() {
        let result = Solution::is_match("aab".to_string(), "c*a*b".to_string());
        assert_eq!(result, true);
    }

    #[test]
    fn case018_multiple_zero_patterns() {
        let result = Solution::is_match("aaa".to_string(), "ab*a*c*d*e*f*a".to_string());
        assert_eq!(result, true);
    }

    #[test]
    fn case019_simplest_zero_pattern() {
        let result = Solution::is_match("aa".to_string(), "a*a".to_string());
        assert_eq!(result, true);
    }

    #[test]
    fn case020_simplest_zero_pattern_interaction() {
        let result = Solution::is_match("aa".to_string(), "a*b*a".to_string());
        assert_eq!(result, true);
    }

    #[test]
    fn case021_fucked_up_lookup() {
        let result = Solution::is_match("aaacacaab".to_string(), ".*ab".to_string());
        assert_eq!(result, true);
    }

    #[test]
    fn case022_zero_or_more_at_the_end() {
        let result = Solution::is_match("a".to_string(), "ab*".to_string());
        assert_eq!(result, true);
    }
}
