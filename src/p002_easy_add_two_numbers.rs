#![allow(unused)]

pub struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn add_two_numbers(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        Self::add_numbers_recursive(&l1, &l2, 0)
    }

    fn add_numbers_recursive(
        i1: &Option<Box<ListNode>>,
        i2: &Option<Box<ListNode>>,
        overflow: i32,
    ) -> Option<Box<ListNode>> {
        if i1.is_none() && i2.is_none() {
            if overflow != 0 {
                return Some(Box::new(ListNode::new(overflow)));
            }

            return None;
        }

        let d1 = i1.as_ref().map(|v| v.val).unwrap_or(0);
        let d2 = i2.as_ref().map(|v| v.val).unwrap_or(0);

        let sum = d1 + d2 + overflow;

        let digit = sum % 10;

        let mut node = ListNode::new(digit);
        let overflow = sum / 10;

        let mut n1: &Option<Box<ListNode>> = &None;
        if let Some(i1ref) = i1.as_ref() {
            n1 = &i1ref.next;
        }
        let mut n2: &Option<Box<ListNode>> = &None;
        if let Some(i2ref) = i2.as_ref() {
            n2 = &i2ref.next;
        }

        node.next = Self::add_numbers_recursive(n1, n2, overflow);

        Some(Box::new(node))
    }

    pub fn from_string(s: &str) -> Option<Box<ListNode>> {
        let iter = s.chars().rev();

        let head = Self::from_iter(iter);

        head
    }

    fn from_iter(mut iter: impl Iterator<Item = char>) -> Option<Box<ListNode>> {
        let d = iter.next();

        match d {
            Some(val) => {
                let digit = val.to_string().parse::<i32>().unwrap();
                let mut node = ListNode::new(digit);
                node.next = Self::from_iter(iter);

                return Some(Box::new(node));
            }
            None => return None,
        }
    }
}

// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

#[cfg(test)]
mod test {
    use super::Solution;
    use crate::p002_easy_add_two_numbers::ListNode;

    #[test]
    fn case1() {
        assert_eq!(
            Solution::add_two_numbers(Solution::from_string("10"), Solution::from_string("10")),
            Solution::from_string("20")
        );
    }

    #[test]
    fn case2() {
        assert_eq!(
            Solution::add_two_numbers(Solution::from_string("10"), Solution::from_string("1000")),
            Solution::from_string("1010")
        );
    }

    #[test]
    fn case3() {
        assert_eq!(
            Solution::add_two_numbers(Solution::from_string("893"), Solution::from_string("9878")),
            Solution::from_string("10771")
        );
    }

    #[test]
    fn case4() {
        assert_eq!(
            Solution::add_two_numbers(Solution::from_string("500"), Solution::from_string("500")),
            Solution::from_string("1000")
        );
    }

    #[test]
    fn case5() {
        assert_eq!(
            Solution::add_two_numbers(Solution::from_string("9"), Solution::from_string("9")),
            Solution::from_string("18")
        );
    }
}
