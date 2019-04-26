/*
 * @lc app=leetcode id=226 lang=rust
 *
 * [226] Invert Binary Tree
 *
 * https://leetcode.com/problems/invert-binary-tree/description/
 *
 * algorithms
 * Easy (57.48%)
 * Total Accepted:    314.6K
 * Total Submissions: 545.3K
 * Testcase Example:  '[4,2,7,1,3,6,9]'
 *
 * Invert a binary tree.
 *
 * Example:
 *
 * Input:
 *
 *
 * ⁠    4
 * ⁠  /   \
 * ⁠ 2     7
 * ⁠/ \   / \
 * 1   3 6   9
 *
 * Output:
 *
 *
 * ⁠    4
 * ⁠  /   \
 * ⁠ 7     2
 * ⁠/ \   / \
 * 9   6 3   1
 *
 * Trivia:
 * This problem was inspired by this original tweet by Max Howell:
 *
 * Google: 90% of our engineers use the software you wrote (Homebrew), but you
 * can’t invert a binary tree on a whiteboard so f*** off.
 *
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
use crate::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    pub fn invert_tree(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        Solution::invert_helper(root.as_ref());
        root
    }
    fn invert_helper(root: Option<&Rc<RefCell<TreeNode>>>) {
        if let Some(ref node) = root {
            let left = Solution::invert_tree(node.borrow().right.clone());
            let right = Solution::invert_tree(node.borrow().left.clone());
            node.borrow_mut().left = left;
            node.borrow_mut().right = right;
        }
    }
}

pub struct Solution;

#[cfg(test)]

mod test {
    use super::Solution;
    use crate::btree;
    #[test]
    fn it_works() {
        assert_eq!(
            Solution::invert_tree(btree![4, 2, 7, 1, 3, 6, 9]),
            btree![4, 7, 2, 9, 6, 3, 1]
        );
    }
}
