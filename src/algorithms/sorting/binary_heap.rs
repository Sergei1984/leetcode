#![allow(unused)]

use std::fmt::Debug;

pub struct Solution;

impl Solution {
    pub fn sort<T: PartialOrd>(input: &mut Vec<T>) {}
}

pub struct MyBinaryHeap<T: PartialOrd + Debug> {
    items: Vec<T>,
}

impl<T: PartialOrd + Debug> MyBinaryHeap<T> {
    pub fn new() -> Self {
        MyBinaryHeap { items: vec![] }
    }

    pub fn from_vec<TItem: PartialOrd + Clone + Debug>(items: &[TItem]) -> MyBinaryHeap<TItem> {
        let mut result = MyBinaryHeap {
            items: items.to_vec(),
        };

        result.re_heapify();

        result
    }

    pub fn top(&self) -> Option<&T> {
        self.items.get(self.items.len() - 1)
    }

    pub fn pop(&mut self) -> Option<T> {
        let result = self.items.pop();
        if self.items.len() > 0 {
            self.re_heapify();
        }

        result
    }

    pub fn to_sorted(mut self) -> Vec<T> {
        self.collect()
    }

    fn parent(&self, of: usize) -> Option<usize> {
        let idx = self.items.len() - 1 - of;
        let idx_one_base = (idx + 1) / 2;
        if idx_one_base <= 1 {
            return None;
        }

        let parent_index = idx_one_base - 1; // Shift to 1-base first and then to 0-base after

        if parent_index >= self.items.len() {
            return None;
        }

        Some(self.items.len() - 1 - parent_index) // Revert direction
    }

    fn left(&self, of: usize) -> Option<usize> {
        let idx = self.items.len() - 1 - of;
        let left_index = (idx + 1) * 2 - 1; // Shift to 1-base index first then to 0-base after
        if left_index >= self.items.len() {
            return None;
        }

        Some(self.items.len() - 1 - left_index)
    }

    fn right(&self, of: usize) -> Option<usize> {
        let idx = self.items.len() - 1 - of;
        let right_index = (idx + 1) * 2 + 1 - 1; // Shift to 1-base index first then to 0-base after
        if right_index >= self.items.len() {
            return None;
        }

        Some(self.items.len() - 1 - right_index)
    }

    fn heapify(&mut self, i: usize) {
        let l_opt = self.left(i);
        let r_opt = self.right(i);

        let mut largest_idx = i;

        if let Some(l) = l_opt {
            if self.items[l].gt(&self.items[i]) {
                largest_idx = l;
            }
        }

        if let Some(r) = r_opt {
            if self.items[r].gt(&self.items[largest_idx]) {
                largest_idx = r;
            }
        }

        if largest_idx != i {
            self.items.swap(largest_idx, i);
            self.heapify(largest_idx);
        }
    }

    fn re_heapify(&mut self) {
        for i in self.items.len() / 2..self.items.len() {
            self.heapify(i);
        }

        println!("{:?}", self.items);
    }
}

impl<T: PartialOrd + Debug> Iterator for MyBinaryHeap<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.pop()
    }
}

#[cfg(test)]
mod test {

    use super::{MyBinaryHeap, Solution};

    #[test]
    pub fn new_001() {
        let heap =
            MyBinaryHeap::<i32>::from_vec::<i32>(&vec![13, 19, 9, 5, 12, 8, 7, 4, 21, 2, 6, 11]);

        assert_eq!(Some(&21), heap.top());
    }

    #[test]
    pub fn left_right_001() {
        let heap =
            MyBinaryHeap::<i32>::from_vec::<i32>(&vec![13, 19, 9, 5, 12, 8, 7, 4, 21, 2, 6, 11]);

        let l = heap.left(11);
        assert_eq!(Some(10), l);

        let r = heap.right(11);
        assert_eq!(Some(9), r);
    }

    #[test]
    pub fn case_001() {
        let mut heap =
            MyBinaryHeap::<i32>::from_vec::<i32>(&vec![13, 19, 9, 5, 12, 8, 7, 4, 21, 2, 6, 11]);

        let input = heap.to_sorted();

        assert_eq!(input, vec![21, 19, 13, 12, 11, 9, 8, 7, 6, 5, 4, 2]);
    }

    #[test]
    pub fn case_002() {
        let mut input = vec![13];

        Solution::sort(&mut input);

        assert_eq!(input, vec![13]);
    }

    #[test]
    pub fn case_003() {
        let mut input = vec![13, 12];

        Solution::sort(&mut input);

        assert_eq!(input, vec![13, 12]);
    }
}
