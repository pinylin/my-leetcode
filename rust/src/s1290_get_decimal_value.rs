/*
 * @lc app=leetcode.cn id=1290 lang=rust
 *
 * [1290] 二进制链表转整数
 */
use crate::ListNode;
// @lc code=start
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
    pub fn get_decimal_value(head: Option<Box<ListNode>>) -> i32 {
        let mut cur = &head;
        let mut res: i32 = 0;
        while let Some(c) = cur {
            res <<= 1;
            res |= c.val;
            cur = &c.next;
        }
        return res;
    }
}
// @lc code=end

pub struct Solution;

#[cfg(test)]
mod test {
    use super::Solution;
    use crate::linkedlist;

    #[test]
    fn it_works() {
        assert_eq!(
            Solution::get_decimal_value(linkedlist![1, 0, 0, 1, 0, 0, 1, 1, 1, 0, 0, 0, 0, 0, 0]),
            18880
        );

        assert_eq!(Solution::get_decimal_value(linkedlist![1, 0, 1]), 5);
    }
}
