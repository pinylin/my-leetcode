/*
 * @lc app=leetcode id=530 lang=rust
 *
 * [530] Minimum Absolute Difference in BST
 */
// Definition for a binary tree node.
// #[derive(Debug, PartialEq, Eq)]
// pub struct TreeNode {
//   pub val: i32,
//   pub left: Option<Rc<RefCell<TreeNode>>>,
//   pub right: Option<Rc<RefCell<TreeNode>>>,
// }
//
// impl TreeNode {
//   #[inline]
//   pub fn new(val: i32) -> Self {
//     TreeNode {
//       val,
//       left: None,
//       right: None
//     }
//   }
// }
use leetcode_prelude::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn get_minimum_difference(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let v = Solution::inorder_traversal(root);
        let mut n = i32::max_value();
        for i in 0..v.len() - 1 {
            n = std::cmp::min((v[i] - v[i + 1]).abs(), n);
        }
        n
    }
    pub fn inorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        if root.is_none() {
            return vec![];
        }
        let mut v = Vec::new();
        let left = root.as_ref().unwrap().borrow().left.clone();
        let right = root.as_ref().unwrap().borrow().right.clone();
        if left.is_some() {
            v.extend(Self::inorder_traversal(left));
        }
        v.push(root.unwrap().borrow().val);
        if right.is_some() {
            v.extend(Self::inorder_traversal(right));
        }
        v
    }
}

pub struct Solution;

#[cfg(test)]

mod test {
    use super::Solution;
    use crate::btree;
    #[test]
    fn it_works() {
        assert_eq!(Solution::get_minimum_difference(btree![1, null, 3, 2]), 1);
    }
}
