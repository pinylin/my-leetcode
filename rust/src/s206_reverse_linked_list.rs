/*
 * @lc app=leetcode id=206 lang=rust
 *
 * [206] Reverse Linked List
 *
 * https://leetcode.com/problems/reverse-linked-list/description/
 *
 * algorithms
 * Easy (52.90%)
 * Total Accepted:    557.2K
 * Total Submissions: 1M
 * Testcase Example:  '[1,2,3,4,5]'
 *
 * Reverse a singly linked list.
 *
 * Example:
 *
 *
 * Input: 1->2->3->4->5->NULL
 * Output: 5->4->3->2->1->NULL
 *
 *
 * Follow up:
 *
 * A linked list can be reversed either iteratively or recursively. Could you
 * implement both?
 *
 */
// Definition for singly-linked list.
// #[derive(PartialEq, Eq, Clone, Debug)]
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

impl Solution {
    pub fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut curr = head;
        let mut next = None;
        while let Some(mut inner) = curr {
            curr = inner.next.take();
            inner.next = next;
            next = Some(inner);
        }
        next
    }
}

pub struct Solution;

#[cfg(test)]
mod test {
    use super::Solution;
    use crate::linkedlist;

    #[test]
    fn it_works() {
        assert_eq!(
            Solution::reverse_list(linkedlist![1, 1, 2]),
            linkedlist![2, 1, 1]
        );
        assert_eq!(
            Solution::reverse_list(linkedlist![1, 4, 2, 3, 3]),
            linkedlist![3, 3, 2, 4, 1]
        );
    }
}
