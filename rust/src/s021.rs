/*
 * @lc app=leetcode id=21 lang=rust
 *
 * [21] Merge Two Sorted Lists
 *
 * https://leetcode.com/problems/merge-two-sorted-lists/description/
 *
 * algorithms
 * Easy (45.97%)
 * Total Accepted:    523.1K
 * Total Submissions: 1.1M
 * Testcase Example:  '[1,2,4]\n[1,3,4]'
 *
 * Merge two sorted linked lists and return it as a new list. The new list
 * should be made by splicing together the nodes of the first two lists.
 * 
 * Example:
 * 
 * Input: 1->2->4, 1->3->4
 * Output: 1->1->2->3->4->4
 * 
 * 
 */
// Definition for singly-linked list.
// #[derive(PartialEq, Eq, Debug)]
// pub struct ListNode {
//   pub val: i32,
//   pub next: Option<Box<ListNode>>
// }
// 
// impl ListNode {
//   #[inline]
//   fn new(val: i32) -> Self {
//     ListNode {
//       next: None,
//       val
//     }
//   }
// }

use crate::ListNode;

use std::mem;

impl Solution {
    // @Aloxaf 大佬真是厉害，我就不会去想这种操作
    pub fn merge_two_lists(mut l1: Option<Box<ListNode>>, mut l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut dummy = Box::new(ListNode::new(0));
        let mut cur = &mut dummy;

        let (l1, l2) = (&mut l1, &mut l2);
        while let(Some(n1), Some(n2)) = (l1.as_ref(), l2.as_ref()) {
            let (v1, v2) = (n1.val, n2.val);
            if v1 < v2 {
                mem::swap(&mut cur.next, l1);
                mem::swap(&mut cur.next.as_mut().unwrap().next, l1);
            } else {
                mem::swap(&mut cur.next, l2);
                mem::swap(&mut cur.next.as_mut().unwrap().next, l2);
            }
            cur = cur.next.as_mut().unwrap();
        }

        if l1.is_some() {
            mem::swap(&mut cur.next, l1);
        } else {
            mem::swap(&mut cur.next, l2);
        }

        dummy.next
    }
}

pub struct Solution;

#[cfg(test)]
mod test {
    use super::Solution;
    use crate::linkedlist;

    #[test]
    fn it_works() {
        let l1 = linkedlist![1, 2, 2147483647];
        let l2 = linkedlist![1, 2, 4];
        assert_eq!(
            Solution::merge_two_lists(l1, l2),
            linkedlist![1, 1, 2, 2, 4, 2147483647]
        );
    }
}

