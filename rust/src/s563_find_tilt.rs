/*
 * @lc app=leetcode.cn id=563 lang=rust
 *
 * [563] 二叉树的坡度
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
    pub fn find_tilt(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let (_, tilt) = Self::tilt_helper(&root);
        tilt
    }

    fn tilt_helper(root: &Option<Rc<RefCell<TreeNode>>>) -> (i32, i32) {
        if let Some(node) = root {
            let node = node.borrow();
            let left = Self::tilt_helper(&node.left);
            let right = Self::tilt_helper(&node.right);
            return (
                left.0 + right.0 + node.val,
                left.1 + right.1 + (left.0 - right.0).abs(),
            );
        }
        (0, 0)
    }
}
// @lc code=end

pub struct Solution {}
#[cfg(test)]
mod test {
    use super::Solution;
    use crate::btree;

    #[test]
    fn it_works() {
        assert_eq!(Solution::find_tilt(btree![4, 2, 9, 3, 5, null, 7]), 15);
        assert_eq!(Solution::find_tilt(btree![21, 7, 14, 1, 1, 2, 2, 3, 3]), 9);
    }
}
