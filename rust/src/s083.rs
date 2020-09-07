/*
 * @lc app=leetcode id=83 lang=rust
 *
 * [83] Remove Duplicates from Sorted List
 *
 * https://leetcode.com/problems/remove-duplicates-from-sorted-list/description/
 *
 * algorithms
 * Easy (41.95%)
 * Total Accepted:    306.8K
 * Total Submissions: 730.3K
 * Testcase Example:  '[1,1,2]'
 *
 * Given a sorted linked list, delete all duplicates such that each element
 * appear only once.
 *
 * Example 1:
 *
 *
 * Input: 1->1->2
 * Output: 1->2
 *
 *
 * Example 2:
 *
 *
 * Input: 1->1->2->3->3
 * Output: 1->2->3
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

impl Solution {
    // refer:  https://github.com/Aloxaf/LeetCode-Rust/blob/master/src/remove_duplicates_from_sorted_list.rs
    pub fn delete_duplicates(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut p = &mut head;
        while let Some(cur) = p {
            while let Some(next) = cur.next.as_mut() {
                if cur.val != next.val {
                    break;
                };
                let next_next = std::mem::replace(&mut next.next, None);
                let _ = std::mem::replace(&mut cur.next, next_next);
            }
            p = &mut cur.next;
        }
        head
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::Solution;
    use crate::linkedlist;

    #[test]
    fn it_works() {
        assert_eq!(
            Solution::delete_duplicates(linkedlist![1, 1, 2]),
            linkedlist![1, 2]
        );
        assert_eq!(
            Solution::delete_duplicates(linkedlist![1, 1, 2, 3, 3]),
            linkedlist![1, 2, 3]
        );
        assert_eq!(
            Solution::delete_duplicates(linkedlist![1, 1, 1]),
            linkedlist![1]
        );
    }
}
