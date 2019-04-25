/*
 * @lc app=leetcode id=234 lang=rust
 *
 * [234] Palindrome Linked List
 *
 * https://leetcode.com/problems/palindrome-linked-list/description/
 *
 * algorithms
 * Easy (35.37%)
 * Total Accepted:    247.1K
 * Total Submissions: 691.6K
 * Testcase Example:  '[1,2]'
 *
 * Given a singly linked list, determine if it is a palindrome.
 * 
 * Example 1:
 * 
 * 
 * Input: 1->2
 * Output: false
 * 
 * Example 2:
 * 
 * 
 * Input: 1->2->2->1
 * Output: true
 * 
 * Follow up:
 * Could you do it in O(n) time and O(1) space?
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
impl Solution {
    pub fn is_palindrome(mut head: Option<Box<ListNode>>) -> bool {
        if head.is_none() {
            return true;
        }
        let mut count = 0;
        {
            let mut cur = head.as_ref();
            while let Some(node) = cur.take() {
                count += 1;
                cur = node.next.as_ref();
            }
        }
        if count < 2 { return true; }
        count = if count % 2 == 0 {
            count/2
        } else {
            count/2+1
        };
        let mut mid = &mut head;
        for _ in 0..count {
            mid = &mut mid.as_mut().unwrap().next;
        }
        let mut cur = mid.take();
        let mut prev = None;
        while let Some(mut cur_inner) = cur.take() {
            let next = cur_inner.next.take();
            cur_inner.next = prev.take();
            prev = Some(cur_inner);
            cur = next;
        }
        let mut t_head = prev.as_ref();
        let mut p_head = head.as_ref();
        while let Some(t) = t_head.take()  {
            if let Some(p) = p_head.take() {
                if p.val != t.val {
                    return false;
                } 
                p_head = p.next.as_ref(); 
            } else {
                return false;
            }
            t_head = t.next.as_ref();
        }
        true

    }

}

