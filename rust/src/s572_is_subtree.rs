/*
 * @lc app=leetcode.cn id=572 lang=rust
 *
 * [572] 另一个树的子树
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
// @lc code=start
use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn is_subtree(s: Option<Rc<RefCell<TreeNode>>>, t: Option<Rc<RefCell<TreeNode>>>) -> bool {
        if Self::is_same_tree(s.clone(), t.clone()) {
            return true;
        }
        match (s, t) {
            (Some(s), Some(t)) => {
                Self::is_subtree(s.borrow().left.clone(), Some(t.clone()))
                    || Self::is_subtree(s.borrow().right.clone(), Some(t.clone()))
            }
            (None, None) => return true,
            _ => return false,
        }
    }

    pub fn is_same_tree(
        p: Option<Rc<RefCell<TreeNode>>>,
        q: Option<Rc<RefCell<TreeNode>>>,
    ) -> bool {
        match (p, q) {
            (Some(p), Some(q)) => p == q,
            (None, None) => true,
            _ => false,
        }
    }
}
// @lc code=end

pub struct Solution {}
