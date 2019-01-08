#[derive(PartialEq, Eq, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

struct Solution {}

impl Solution {
    pub fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let (mut l1, mut l2) = (l1, l2);
        let mut c = Some(Box::new(ListNode::new(0)));
        let mut tail = &mut c;
        while l1.is_some() || l2.is_some() {
            let a = l1.take().unwrap_or(Box::new(ListNode::new(0)));
            let b = l2.take().unwrap_or(Box::new(ListNode::new(0)));
            tail = match tail.as_mut() {
                Some(inner_box) => {
                    let sum = a.val + b.val + inner_box.val;
                    let carry = sum / 10;
                    inner_box.val = sum % 10;
                    inner_box.next = if a.next.is_none() && b.next.is_none() && carry == 0 {
                        None
                    } else {
                        Some(Box::new(ListNode::new(carry)))
                    };
                    &mut inner_box.next
                }
                _ => unreachable!(),
            };
            l1 = a.next;
            l2 = b.next;
        }
        c
    }
}

// TODO linkedlist macro
macro_rules! linkedlist {
($($x:expr),*) => {
    {
        let mut v = Vec::new();
        // 重复开始：
        $(
            // 每次重复将包含如下元素，其中
            // “$element”将被替换成其相应的展开...
            v.push(Some(Box::new(ListNode::new($x))));
        )*

        for (i, n) in &mut v.iter().enumerate()  {
            match v.get(i+1) {
                Some(node) => n.next = node,
                None => break,
            }
        }
        match v.get(0) {
            Some(node) => return node,
            None => return Some(Box::new(ListNode::new(0))),
        } 
    }
};
}

#[cfg(test)]
mod test {
    use super::{Solution, ListNode, linkedlist};

    #[test]
    fn test() {
        let l1 = linkedlist![2, 4, 3];
        let l2 = linkedlist![5, 6, 4];
        assert_eq!(Solution::add_two_numbers(l1, l2), linkedlist![7, 0, 8]);

        assert_eq!(Solution::add_two_numbers(linkedlist![5], linkedlist![5]), linkedlist![0, 1]);
    }
}