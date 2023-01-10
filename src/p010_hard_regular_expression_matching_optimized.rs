pub struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn is_match(s: String, p: String) -> bool {
        let input = s.as_bytes();
        let pattern = Self::parse_regex(p.as_bytes());

        return Self::match_recursive(input, pattern);
    }

    pub fn match_recursive(input: &[u8], pattern: Option<Box<PatternPair>>) -> bool {
        let _input_str = String::from_utf8(input.to_vec()).unwrap();
        let _fixed_str = pattern
            .as_ref()
            .map(|p| p.fixed.map(|f| String::from_utf8(f.to_vec()).unwrap()))
            .flatten();
        let _repeatable_str = pattern
            .as_ref()
            .map(|p| p.repeatable.map(|f| String::from_utf8(f.to_vec()).unwrap()))
            .flatten();

        if let Some(p) = pattern {
            if let Some(fixed) = p.fixed {
                if fixed.len() > input.len() {
                    return false;
                }
                if let Some(repeatable) = p.repeatable {
                    let mut fixed_match_index = 0;

                    while let Some(idx) = Self::index_of_pattern(input, fixed, fixed_match_index) {
                        let repeatable_part = &input[0..idx];

                        if input.len() >= idx + fixed.len()
                            && Self::match_repeatable(repeatable_part, repeatable)
                        {
                            let rest_matches = Self::match_recursive(
                                &input[idx + fixed.len()..],
                                p.next_pattern.clone(),
                            );

                            if rest_matches {
                                return true;
                            }
                        }

                        fixed_match_index = idx + 1;
                    }

                    return false;
                } else {
                    // if there is no repeatable part, fixed part must match the start of string
                    let index = Self::index_of_pattern(input, fixed, 0);

                    let matches_start = index.map(|v| v == 0).unwrap_or(false);
                    if !matches_start || fixed.len() > input.len() {
                        // input has some symbols before fixed part
                        return false;
                    }

                    return Self::match_recursive(&input[fixed.len()..], p.next_pattern);
                }
            } else {
                return Self::match_repeatable(input, p.repeatable.unwrap()); // if there is no fixed part pattern matches if repeatable part matches full input
            }
        } else {
            return input.len() == 0; // must be end of string on end of pattern
        }
    }

    // Check if input fully matched by the pattern or pattern prefix
    pub fn match_repeatable(input: &[u8], pattern: &[u8]) -> bool {
        if input.len() == 0 {
            return true;
        }

        if pattern.len() == 0 {
            return false;
        }

        let match_symbol = pattern[0];

        if match_symbol == b'.' {
            return true;
        }

        for i in 0..input.len() {
            let c = input[i];
            if c != match_symbol {
                return Self::match_repeatable(&input[i..], &pattern[2..]);
            }
        }

        true
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
        if regex.len() < 2 {
            return (None, regex);
        }

        let mut i = 1;
        while i < regex.len() {
            // a*b*cd
            if regex[i] != b'*' {
                let repeatable_end = i - 1;
                if repeatable_end == 0 {
                    return (None, regex);
                } else {
                    return (Some(&regex[0..repeatable_end]), &regex[repeatable_end..]);
                }
            }

            i = i + 2;
        }
        if regex.len() % 2 == 1 {
            return (Some(&regex[0..regex.len() - 1]), &regex[regex.len() - 1..]);
        }

        return (Some(&regex), &regex[regex.len()..]);
    }

    pub fn index_of_pattern(input: &[u8], pattern: &[u8], start_index: usize) -> Option<usize> {
        assert_ne!(pattern.len(), 0);

        if start_index >= input.len() {
            return None;
        }

        if input.len() == 0 {
            return None;
        }

        'outer: for i in start_index..input.len() {
            if input[i] == pattern[0] || pattern[0] == b'.' {
                for k in 1..pattern.len() {
                    if let Some(k_input) = input.get(i + k) {
                        if *k_input != pattern[k] && pattern[k] != b'.' {
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

#[derive(Debug, Clone)]
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
    fn capture_repeatable_4() {
        let (pattern, rest) = Solution::capture_repeatable("a".as_bytes());

        assert_eq!(true, pattern.is_none());
        assert_eq!(rest, "a".as_bytes());
    }

    #[test]
    fn capture_repeatable_5() {
        let (pattern, rest) = Solution::capture_repeatable(".*c".as_bytes());

        assert_eq!(false, pattern.is_none());
        assert_eq!(pattern.unwrap(), ".*".as_bytes());
        assert_eq!(rest, "c".as_bytes());
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
    fn parse_regex_only_fixed() {
        let parsed = Solution::parse_regex("a".as_bytes());

        assert_eq!(true, parsed.is_some());

        let regex = parsed.unwrap();

        assert_eq!(None, regex.repeatable);
        assert_ne!(None, regex.fixed);
        assert_eq!(b"a", regex.fixed.unwrap());
        assert_eq!(true, regex.next_pattern.is_none());
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
    fn parse_regex_001() {
        let parsed = Solution::parse_regex(".*c".as_bytes());
        assert_eq!(true, parsed.is_some());

        let regex = parsed.unwrap();

        assert_eq!(true, regex.repeatable.is_some());
        assert_eq!(true, regex.fixed.is_some());
        assert_eq!(true, regex.next_pattern.is_none());

        assert_eq!(b".*", regex.repeatable.unwrap());
        assert_eq!(b"c", regex.fixed.unwrap());
    }

    #[test]
    fn match_repeatable_simple() {
        assert_eq!(
            Solution::match_repeatable("abdabeabc".as_bytes(), ".*".as_bytes()),
            true
        );
    }

    #[test]
    fn match_repeatable_fixed_simple() {
        assert_eq!(
            Solution::match_repeatable("aaabbb".as_bytes(), "a*b*".as_bytes()),
            true
        );
    }

    #[test]
    fn match_repeatable_fixed_simple_not_match() {
        assert_eq!(
            Solution::match_repeatable("bbbaaa".as_bytes(), "a*b*".as_bytes()),
            false
        );
    }

    #[test]
    fn index_of_1() {
        assert_eq!(
            Solution::index_of_pattern("abcd".as_bytes(), "bc".as_bytes(), 0),
            Some(1)
        );
    }

    #[test]
    fn index_of_2() {
        assert_eq!(
            Solution::index_of_pattern("abdabeabc".as_bytes(), "abc".as_bytes(), 0),
            Some(6)
        );
    }

    #[test]
    fn index_of_3() {
        assert_eq!(
            Solution::index_of_pattern("abc".as_bytes(), "abc".as_bytes(), 0),
            Some(0)
        );
    }

    #[test]
    fn index_of_4() {
        assert_eq!(
            Solution::index_of_pattern("abc".as_bytes(), "abcdef".as_bytes(), 0),
            Some(0)
        );
    }

    #[test]
    fn index_of_pattern_1() {
        assert_eq!(
            Solution::index_of_pattern("abcd".as_bytes(), "b.".as_bytes(), 0),
            Some(1)
        );
    }

    #[test]
    fn index_of_pattern_2() {
        assert_eq!(
            Solution::index_of_pattern("abcd".as_bytes(), "..".as_bytes(), 0),
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

    #[test]
    fn case031_leetcode_test() {
        let result = Solution::is_match("a".to_string(), ".*..a*".to_string());
        assert_eq!(result, false);
    }

    #[test]
    fn case032_leetcode_test() {
        let result = Solution::is_match(
            "bccbbabcaccacbcacaa".to_string(),
            ".*b.*c*.*.*.c*a*.c".to_string(),
        );
        assert_eq!(result, false);
    }
}
