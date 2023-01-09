pub struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn is_match(s: String, p: String) -> bool {
        let input = s.as_bytes();
        let pattern = Self::parse_regex(input);

        return Self::match_recursive(input, pattern);
    }

    pub fn match_recursive(input: &[u8], pattern: Option<Box<PatternPair>>) -> bool {
        false
    }

    pub fn parse_regex<'a>(regex: &'a [u8]) -> Option<Box<PatternPair<'a>>> {
        if regex.len() == 0 {
            return None;
        }

        let (repeatable_part, next) = Self::capture_repeatable(regex);
        let (fixed_part, rest) = Self::capture_fixed(next);

        Some(Box::new(PatternPair {
            fixed: fixed_part,
            repeatable: repeatable_part,
            next_pattern: Self::parse_regex(rest),
        }))
    }

    // Captures fixed part of regex (first tuple member) and returns rest of slice (second tuple member)
    pub fn capture_fixed<'a>(regex: &'a [u8]) -> (Option<&'a [u8]>, &'a [u8]) {
        if regex.len() == 0 {
            return (None, regex);
        }

        if let Some(asterisk_index) = regex.iter().position(|s| *s == b'*') {
            if asterisk_index < 2 {
                return (None, regex); // No fixed part
            } else {
                return (
                    Some(&regex[0..asterisk_index - 1]),
                    &regex[asterisk_index - 1..],
                );
            }
        } else {
            return (Some(regex), &regex[regex.len()..]); // No repeatable part
        }
    }

    // Captures repeatable part of regex (first tuple member) and returns rest of slice (second tuple member)
    pub fn capture_repeatable<'a>(regex: &'a [u8]) -> (Option<&'a [u8]>, &'a [u8]) {
        if regex.len() == 0 {
            return (None, regex);
        }
        for i in 0..regex.len() {
            if regex[i] == b'*' {
                continue;
            }

            let next = regex.get(i + 1);
            if let Some(next_char) = next {
                if *next_char != b'*' {
                    if i < 2 {
                        return (None, regex); // No repeatable part
                    } else {
                        return (Some(&regex[0..i]), &regex[i..]);
                    }
                }
            }
        }

        return (Some(&regex), &regex[regex.len()..]);
    }

    pub fn index_of(input: &[u8], pattern: &[u8]) -> Option<usize> {
        assert_ne!(pattern.len(), 0);

        if input.len() == 0 {
            return None;
        }

        'outer: for i in 0..input.len() {
            if input[i] == pattern[0] {
                for k in 1..pattern.len() {
                    if let Some(k_input) = input.get(i + k) {
                        if *k_input != pattern[k] {
                            continue 'outer;
                        }
                    } else {
                        break;
                    }
                }

                return Some(i);
            }
        }

        None
    }
}

#[derive(Debug)]
pub struct PatternPair<'a> {
    pub fixed: Option<&'a [u8]>,
    pub repeatable: Option<&'a [u8]>,

    pub next_pattern: Option<Box<PatternPair<'a>>>,
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn capture_fixed_1() {
        let (pattern, rest) = Solution::capture_fixed("abc*d*efg*h*jk".as_bytes());

        assert_eq!(
            "ab".to_string(),
            pattern
                .map(|b| String::from_utf8(b.to_vec()).unwrap())
                .unwrap_or_default()
        );

        assert_eq!(
            "c*d*efg*h*jk".to_string(),
            String::from_utf8(rest.to_vec()).unwrap()
        );
    }

    #[test]
    fn capture_fixed_2() {
        let (pattern, rest) = Solution::capture_fixed("c*d*efg*h*jk".as_bytes());

        assert_eq!(None, pattern);

        assert_eq!(
            "c*d*efg*h*jk".to_string(),
            String::from_utf8(rest.to_vec()).unwrap()
        );
    }

    #[test]
    fn capture_fixed_3() {
        let (pattern, rest) = Solution::capture_fixed("cjk".as_bytes());

        assert_eq!(
            "cjk".to_string(),
            pattern
                .map(|b| String::from_utf8(b.to_vec()).unwrap())
                .unwrap_or_default()
        );

        assert_eq!(0, rest.len());
    }

    #[test]
    fn capture_repeatable_1() {
        let (pattern, rest) = Solution::capture_repeatable("c*d*efg*h*jk".as_bytes());

        assert_eq!(
            "c*d*".to_string(),
            pattern
                .map(|b| String::from_utf8(b.to_vec()).unwrap())
                .unwrap_or_default()
        );

        assert_eq!(
            "efg*h*jk".to_string(),
            String::from_utf8(rest.to_vec()).unwrap()
        );
    }

    #[test]
    fn capture_repeatable_2() {
        let (pattern, rest) = Solution::capture_repeatable("abc*d*efg*h*jk".as_bytes());

        assert_eq!(None, pattern);

        assert_eq!(
            "abc*d*efg*h*jk".to_string(),
            String::from_utf8(rest.to_vec()).unwrap()
        );
    }

    #[test]
    fn capture_repeatable_3() {
        let (pattern, rest) = Solution::capture_repeatable("c*d*e*f*g*h*".as_bytes());

        assert_eq!(
            "c*d*e*f*g*h*".to_string(),
            pattern
                .map(|b| String::from_utf8(b.to_vec()).unwrap())
                .unwrap_or_default()
        );

        assert_eq!(0, rest.len());
    }

    #[test]
    fn parse_regex() {
        let parsed = Solution::parse_regex("abc*d*efg*h*jk".as_bytes());

        let mut node = &parsed;

        while let Some(n) = node {
            println!(
                "Repeatable: {:?} Fixed: {:?}",
                n.repeatable.map(|b| String::from_utf8(b.to_vec())),
                n.fixed.map(|b| String::from_utf8(b.to_vec())),
            );
            node = &n.next_pattern;
        }
    }

    #[test]
    fn parse_regex_start_repeatable() {
        let parsed = Solution::parse_regex("c*d*efg*h*jk".as_bytes());

        let mut node = &parsed;

        while let Some(n) = node {
            println!(
                "Repeatable: {:?} Fixed: {:?}",
                n.repeatable.map(|b| String::from_utf8(b.to_vec())),
                n.fixed.map(|b| String::from_utf8(b.to_vec())),
            );
            node = &n.next_pattern;
        }
    }

    #[test]
    fn parse_regex_start_fixed_only() {
        let parsed = Solution::parse_regex("cdefghjk".as_bytes());

        let mut node = &parsed;

        while let Some(n) = node {
            println!(
                "Repeatable: {:?} Fixed: {:?}",
                n.repeatable.map(|b| String::from_utf8(b.to_vec())),
                n.fixed.map(|b| String::from_utf8(b.to_vec())),
            );
            node = &n.next_pattern;
        }
    }

    #[test]
    fn parse_regex_start_repeatable_only() {
        let parsed = Solution::parse_regex("c*d*e*f*g*h*j*k*".as_bytes());

        let mut node = &parsed;

        while let Some(n) = node {
            println!(
                "Repeatable: {:?} Fixed: {:?}",
                n.repeatable.map(|b| String::from_utf8(b.to_vec())),
                n.fixed.map(|b| String::from_utf8(b.to_vec())),
            );
            node = &n.next_pattern;
        }
    }

    #[test]
    fn index_of_1() {
        assert_eq!(
            Solution::index_of("abcd".as_bytes(), "bc".as_bytes()),
            Some(1)
        );
    }

    #[test]
    fn index_of_2() {
        assert_eq!(
            Solution::index_of("abdabeabc".as_bytes(), "abc".as_bytes()),
            Some(6)
        );
    }

    #[test]
    fn index_of_3() {
        assert_eq!(
            Solution::index_of("abc".as_bytes(), "abc".as_bytes()),
            Some(0)
        );
    }

    #[test]
    fn index_of_4() {
        assert_eq!(
            Solution::index_of("abc".as_bytes(), "abcdef".as_bytes()),
            Some(0)
        );
    }

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
