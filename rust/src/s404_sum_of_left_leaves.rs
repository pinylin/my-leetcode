/*
 * @lc app=leetcode id=404 lang=rust
 *
 * [404] Sum of Left Leaves
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
    pub fn sum_of_left_leaves(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        Solution::helper(root.as_ref(), 0)
    }
    fn helper(root: Option<&Rc<RefCell<TreeNode>>>, mark: i32) -> i32 {
        match root {
            None => 0,
            Some(ref node) => {
                let cur = node.borrow();
                if cur.left == None && cur.right == None && mark == 1 {
                    return cur.val;
                }
                Solution::helper(cur.left.as_ref(), 1) + Solution::helper(cur.right.as_ref(), 0)
            }
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
        assert_eq!(Solution::sum_of_left_leaves(btree![1, 2, 3, null, 5]), 0);
        assert_eq!(
            Solution::sum_of_left_leaves(btree![3, 9, 20, None, None, 15, 7]),
            24
        );
    }
}
