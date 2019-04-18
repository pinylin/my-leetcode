/*
 * @lc app=leetcode id=203 lang=rust
 *
 * [203] Remove Linked List Elements
 *
 * https://leetcode.com/problems/remove-linked-list-elements/description/
 *
 * algorithms
 * Easy (35.30%)
 * Total Accepted:    216.9K
 * Total Submissions: 609.8K
 * Testcase Example:  '[1,2,6,3,4,5,6]\n6'
 *
 * Remove all elements from a linked list of integers that have value val.
 * 
 * Example:
 * 
 * 
 * Input:  1->2->6->3->4->5->6, val = 6
 * Output: 1->2->3->4->5
 * 
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
    pub fn remove_elements(mut head: Option<Box<ListNode>>, val: i32) -> Option<Box<ListNode>> {
        // let mut p = &mut head;
        // while let Some(cur) = p {
        //     while let Some(next) = cur.next.as_mut() {
        //         if val != next.val {
        //             break;
        //         }
        //         let next_next = std::mem::replace(&mut next.next, None);
        //         std::mem::replace(&mut cur.next, next_next);
        //     }
        //     p = &mut cur.next;
        // }
        // head
        let mut dummy = Some(Box::new(ListNode::new(0)));
        let mut next = dummy.as_mut();
        while let Some(mut inner) = head {
            head = inner.next.take();
            if inner.val != val {
                next.as_mut().unwrap().next = Some(inner);
                next = next.unwrap().next.as_mut();
            }
        }
        dummy.unwrap().next
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
            Solution::remove_elements(linkedlist![1, 1, 2], 1),
            linkedlist![2]
        );
        assert_eq!(
            Solution::remove_elements(linkedlist![1, 1, 2, 3, 3], 3),
            linkedlist![1, 1, 2]
        );
    }
}