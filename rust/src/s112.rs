/*
 * @lc app=leetcode id=112 lang=rust
 *
 * [112] Path Sum
 *
 * https://leetcode.com/problems/path-sum/description/
 *
 * algorithms
 * Easy (37.31%)
 * Total Accepted:    298.2K
 * Total Submissions: 798.3K
 * Testcase Example:  '[5,4,8,11,null,13,4,7,2,null,null,null,1]\n22'
 *
 * Given a binary tree and a sum, determine if the tree has a root-to-leaf path
 * such that adding up all the values along the path equals the given sum.
 *
 * Note: A leaf is a node with no children.
 *
 * Example:
 *
 * Given the below binary tree and sum = 22,
 *
 *
 * ⁠     5
 * ⁠    / \
 * ⁠   4   8
 * ⁠  /   / \
 * ⁠ 11  13  4
 * ⁠/  \      \
 * 7    2      1
 *
 *
 * return true, as there exist a root-to-leaf path 5->4->11->2 which sum is 22.
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
use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;
impl Solution {
    pub fn has_path_sum(root: Option<Rc<RefCell<TreeNode>>>, sum: i32) -> bool {
        if root.is_none() {
            return false;
        }
        let mut que = VecDeque::new();
        que.push_back((0, root.unwrap()));
        while !que.is_empty() {
            if let Some((acc, node)) = que.pop_front() {
                let acc = acc + node.borrow().val;
                if node.borrow().left.is_none() && node.borrow().right.is_none() {
                    if acc == sum {
                        return true;
                    }
                } else {
                    if node.borrow().left.is_some() {
                        que.push_back((acc, node.borrow().left.as_ref().unwrap().clone()))
                    };
                    if node.borrow().right.is_some() {
                        que.push_back((acc, node.borrow().right.as_ref().unwrap().clone()))
                    };
                }
            }
        }
        false
    }
}

use crate::TreeNode;

pub struct Solution;

#[cfg(test)]
mod test {
    use super::Solution;
    use crate::btree;

    #[test]
    fn it_works() {
        assert_eq!(
            Solution::has_path_sum(
                btree![5, 4, 8, 11, null, 13, 4, 7, 2, null, null, null, 1],
                22
            ),
            true
        );
    }
}
