/*
 * @lc app=leetcode id=100 lang=rust
 *
 * [100] Same Tree
 *
 * https://leetcode.com/problems/same-tree/description/
 *
 * algorithms
 * Easy (49.45%)
 * Total Accepted:    358.3K
 * Total Submissions: 722.4K
 * Testcase Example:  '[1,2,3]\n[1,2,3]'
 *
 * Given two binary trees, write a function to check if they are the same or
 * not.
 *
 * Two binary trees are considered the same if they are structurally identical
 * and the nodes have the same value.
 *
 * Example 1:
 *
 *
 * Input:     1         1
 * ⁠         / \       / \
 * ⁠        2   3     2   3
 *
 * ⁠       [1,2,3],   [1,2,3]
 *
 * Output: true
 *
 *
 * Example 2:
 *
 *
 * Input:     1         1
 * ⁠         /           \
 * ⁠        2             2
 *
 * ⁠       [1,2],     [1,null,2]
 *
 * Output: false
 *
 *
 * Example 3:
 *
 *
 * Input:     1         1
 * ⁠         / \       / \
 * ⁠        2   1     1   2
 *
 * ⁠       [1,2,1],   [1,1,2]
 *
 * Output: false
 *
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

    pub fn is_same_tree2(
        p: Option<Rc<RefCell<TreeNode>>>,
        q: Option<Rc<RefCell<TreeNode>>>,
    ) -> bool {
        match (p, q) {
            (Some(p), Some(q)) => {
                p.borrow().val == q.borrow().val
                    && Self::is_same_tree2(p.borrow().left.clone(), q.borrow().left.clone())
                    && Self::is_same_tree2(p.borrow().right.clone(), q.borrow().right.clone())
            }
            (None, None) => true,
            _ => false,
        }
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::Solution;
    use crate::btree;

    #[test]
    fn builtin() {
        assert_eq!(
            Solution::is_same_tree(btree![1, 2, 3], btree![1, 2, 3]),
            true
        );
        assert_eq!(
            Solution::is_same_tree(btree![1, 2], btree![1, null, 2]),
            false
        );
    }

    #[test]
    fn recursive() {
        assert_eq!(
            Solution::is_same_tree2(btree![1, 2, 3], btree![1, 2, 3]),
            true
        );
        assert_eq!(
            Solution::is_same_tree2(btree![1, 2], btree![1, null, 2]),
            false
        );
    }
}

#[cfg(test)]
mod bench {
    extern crate test;
    use self::test::Bencher;
    use super::Solution;
    use crate::btree;

    #[bench]
    fn builtin(b: &mut Bencher) {
        b.iter(|| Solution::is_same_tree(btree![1, 2, 3, 4, 5], btree![1, 2, 3, 4, 5]));
    }

    #[bench]
    fn recursive(b: &mut Bencher) {
        b.iter(|| Solution::is_same_tree2(btree![1, 2, 3, 4, 5], btree![1, 2, 3, 4, 5]));
    }
}
