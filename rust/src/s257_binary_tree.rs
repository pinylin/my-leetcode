/*
 * @lc app=leetcode id=257 lang=rust
 *
 * [257] Binary Tree Paths
 *
 * https://leetcode.com/problems/binary-tree-paths/description/
 *
 * algorithms
 * Easy (45.36%)
 * Total Accepted:    219.4K
 * Total Submissions: 481.5K
 * Testcase Example:  '[1,2,3,null,5]'
 *
 * Given a binary tree, return all root-to-leaf paths.
 * 
 * Note: A leaf is a node with no children.
 * 
 * Example:
 * 
 * 
 * Input:
 * 
 * ⁠  1
 * ⁠/   \
 * 2     3
 * ⁠\
 * ⁠ 5
 * 
 * Output: ["1->2->5", "1->3"]
 * 
 * Explanation: All root-to-leaf paths are: 1->2->5, 1->3
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
use std::rc::Rc;
use std::cell::RefCell;
use crate::TreeNode;
impl Solution {
    pub fn binary_tree_paths(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<String> {
        let mut res = Vec::new();
        if root.is_none() {
            return res;
        }
        Solution::path_helper(root.as_ref(), &mut res, String::new());
        res
    }
    fn path_helper(root: Option<&Rc<RefCell<TreeNode>>>, res: &mut Vec<String>, str_path: String) {
        if let Some(ref node) = root {
            let cur = node.borrow();
            let mut path = str_path;
            if path.len() == 0 {
                path = format!("{}", cur.val)
            } else {
                path = format!("{}->{}", path, cur.val)
            }
            if cur.left == None && cur.right == None {
                res.push(path);
                return
            }
            if cur.left != None {
                Solution::path_helper(cur.left.as_ref(), res, path.clone());
            }
            if cur.right != None {
                Solution::path_helper(cur.right.as_ref(), res, path.clone());
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
        assert_eq!(
            Solution::binary_tree_paths(btree![1, 2, 3, null, 5]), 
            vec!["1->2->5", "1->3"]
        );
    }
}