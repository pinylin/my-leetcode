/*
 * @lc app=leetcode id=437 lang=rust
 *
 * [437] Path Sum III
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
use std::collections::HashMap;
use std::rc::Rc;

#[allow(dead_code)]
impl Solution {
    pub fn path_sum(root: Option<Rc<RefCell<TreeNode>>>, sum: i32) -> i32 {
        let mut map: HashMap<i32, i32> = HashMap::new();
        map.insert(0, 1);
        Solution::backtrace(root, 0, sum, &mut map)
        // let mut res = 0;
        // if root.is_none() {
        //     return res;
        // }
        // Solution::path_helper(root.as_ref(), &mut res, 0, sum);
        // if let Some(ref node) = root {
        //     let cur = node.borrow();
        //     res += Solution::path_sum(cur.left.clone(), sum) + Solution::path_sum(cur.right.clone(), sum)
        // }
        // res
    }
    // my
    fn path_helper(root: Option<&Rc<RefCell<TreeNode>>>, res: &mut i32, path_sum: i32, sum: i32) {
        if let Some(ref node) = root {
            let cur = node.borrow();
            let path = path_sum + cur.val;
            if path == sum {
                *res += 1;
            }

            if cur.left != None {
                Solution::path_helper(cur.left.as_ref(), res, path, sum);
            }
            if cur.right != None {
                Solution::path_helper(cur.right.as_ref(), res, path, sum);
            }
        }
    }
    // best
    fn backtrace(
        root: Option<Rc<RefCell<TreeNode>>>,
        mut sum: i32,
        target: i32,
        map: &mut HashMap<i32, i32>,
    ) -> i32 {
        match root {
            None => 0,
            Some(r) => {
                let root = r.borrow();
                sum += root.val;
                let mut result = *map.get(&(sum - target)).unwrap_or(&0);
                map.insert(sum, 1 + *map.get(&sum).unwrap_or(&0));
                result += Solution::backtrace(root.left.clone(), sum, target, map);
                result += Solution::backtrace(root.right.clone(), sum, target, map);
                map.get_mut(&sum).map(|v| *v -= 1);
                result
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
            Solution::path_sum(btree![10, 5, -3, 3, 2, null, 11, 3, -2, null, 1], 8),
            3
        );
        assert_eq!(
            Solution::path_sum(btree![1, -2, -3, 1, 3, -2, null, -1], -1),
            4
        );
    }
}
