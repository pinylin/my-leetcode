/*
 * @lc app=leetcode id=538 lang=rust
 *
 * [538] Convert BST to Greater Tree
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
use std::rc::Rc;
use std::cell::RefCell;
use crate::TreeNode;
impl Solution {
    pub fn convert_bst(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        let mut sum = 0;
        Self::helper(root.clone(), &mut sum);
        root
    }
    fn helper(root: Option<Rc<RefCell<TreeNode>>>, sum: &mut i32) -> Option<Rc<RefCell<TreeNode>>> {
        if root.is_some() {
            Self::helper(root.as_ref().unwrap().borrow().right.clone(), sum);
            *sum += root.as_ref().unwrap().borrow().val;
            root.as_ref().unwrap().borrow_mut().val = *sum;
            Self::helper(root.as_ref().unwrap().borrow().left.clone(), sum);
        }
        root
    }
}

pub struct Solution;

#[cfg(test)]
mod test {
    use super::Solution;
    use crate::btree;

    #[test]
    fn convert_bst() {
        assert_eq!(
            btree![18, 20, 13],
            Solution::convert_bst(btree![5, 2, 13])
        );
    }
}