#[allow(dead_code)]
pub struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        let mut max_len: usize = 0;

        let mut prev: Option<&String> = None;

        for s in strs.iter() {
            if let Some(p) = prev {
                let diff_len = Self::common_prefix_len(s, p, max_len);

                if diff_len == 0 {
                    return "".to_string();
                }

                max_len = diff_len;
            } else {
                max_len = s.len();
            }

            prev = Some(s);
        }

        if max_len == 0 {
            return "".to_string();
        }

        return strs[0][0..max_len].to_string();
    }

    #[inline]
    fn common_prefix_len(s1: &String, s2: &String, max_common_prefix_len: usize) -> usize {
        let mlen = std::cmp::min(s1.len(), std::cmp::min(s2.len(), max_common_prefix_len));

        let z = s1.chars().take(mlen).zip(s2.chars().take(mlen));

        for (i, (c1, c2)) in z.enumerate() {
            if c1 != c2 {
                return i;
            }
        }

        return mlen;
    }
}

#[cfg(test)]
mod p13_test {
    use super::Solution;

    #[test]
    fn case1() {
        assert_eq!(
            Solution::longest_common_prefix(vec![
                "flower".to_string(),
                "flow".to_string(),
                "flight".to_string()
            ]),
            "fl"
        );
    }

    #[test]
    fn case2() {
        assert_eq!(
            Solution::longest_common_prefix(vec![
                "dog".to_string(),
                "racecar".to_string(),
                "car".to_string()
            ]),
            ""
        );
    }

    #[test]
    fn case3() {
        assert_eq!(Solution::longest_common_prefix(vec!["".to_string()]), "");
    }

    #[test]
    fn case4() {
        assert_eq!(Solution::longest_common_prefix(vec!["a".to_string()]), "a");
    }

    #[test]
    fn case5() {
        assert_eq!(
            Solution::longest_common_prefix(vec!["ab".to_string(), "a".to_string()]),
            "a"
        );
    }
}
