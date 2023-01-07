#[allow(dead_code)]

pub struct Solution;

impl Solution {
    pub fn is_match(s: String, p: String) -> bool {
        let first_pattern = Self::next_pattern(p.as_str());
        return Self::match_recursive(s.as_str(), first_pattern);
    }

    fn match_recursive<'a>(s: &str, p: Option<Pattern<'a>>) -> bool {
        if let Some(pattern) = p {
            let next_char = s.chars().next();

            if let Some(next) = next_char {
                if pattern.match_char == '.' || next == pattern.match_char {
                    return Self::match_recursive(&s[1..], Self::next_pattern(pattern.next_regex));
                } else {
                    return false;
                }
            } else {
                return false;
            }
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
}

fn main() {
    println!("Hello, world!");
}
