use leetcode_prelude::TreeNode;
use std::rc::Rc;
use std::cell::RefCell;

pub fn merge_sort(a: &mut Vec<i32>, b: usize, e: usize) {
    if b < e {
        let m = (b + e) / 2;
        merge_sort(a, b, m);
        merge_sort(a, m + 1, e);
        merge(a, b, m, e);
    }
}
fn merge(a: &mut Vec<i32>, b: usize, m: usize, e: usize) {
    let mut left = a[b..m + 1].to_vec();
    let mut right = a[m + 1..e + 1].to_vec();
    left.reverse();
    right.reverse();
    for k in b..e + 1 {
        if left.is_empty() {
            a[k] = right.pop().unwrap();
            continue;
        }
        if right.is_empty() {
            a[k] = left.pop().unwrap();
            continue;
        }
        if right.last() < left.last() {
            a[k] = right.pop().unwrap();
        } else {
            a[k] = left.pop().unwrap();
        }
    }
}

pub struct Sort{}
impl Sort {
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