/*
 * @lc app=leetcode id=232 lang=rust
 *
 * [232] Implement Queue using Stacks
 *
 * https://leetcode.com/problems/implement-queue-using-stacks/description/
 *
 * algorithms
 * Easy (42.13%)
 * Total Accepted:    144.2K
 * Total Submissions: 337.3K
 * Testcase Example:  '["MyQueue","push","push","peek","pop","empty"]\n[[],[1],[2],[],[],[]]'
 *
 * Implement the following operations of a queue using stacks.
 * 
 * 
 * push(x) -- Push element x to the back of queue.
 * pop() -- Removes the element from in front of queue.
 * peek() -- Get the front element.
 * empty() -- Return whether the queue is empty.
 * 
 * 
 * Example:
 * 
 * 
 * MyQueue queue = new MyQueue();
 * 
 * queue.push(1);
 * queue.push(2);  
 * queue.peek();  // returns 1
 * queue.pop();   // returns 1
 * queue.empty(); // returns false
 * 
 * Notes:
 * 
 * 
 * You must use only standard operations of a stack -- which means only push to
 * top, peek/pop from top, size, and is empty operations are valid.
 * Depending on your language, stack may not be supported natively. You may
 * simulate a stack by using a list or deque (double-ended queue), as long as
 * you use only standard operations of a stack.
 * You may assume that all operations are valid (for example, no pop or peek
 * operations will be called on an empty queue).
 * 
 * 
 */
use std::mem;
struct MyQueue {
    in_stack: Vec<i32>,
    out_stack: Vec<i32>,
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
#[allow(dead_code)]
impl MyQueue {
    /** Initialize your data structure here. */
    fn new() -> Self {
        MyQueue {
            in_stack: Vec::new(),
            out_stack: Vec::new(),
        }
    }
    
    /** Push element x to the back of queue. */
    fn push(&mut self, x: i32) {
        self.in_stack.push(x);
    }
    
    /** Removes the element from in front of queue and returns that element. */
    fn pop(&mut self) -> i32 {
        if  self.out_stack.len() < 1 {
            self.in_stack.reverse();
            mem::swap(&mut self.in_stack, &mut self.out_stack);
        }
        match self.out_stack.pop() {
            Some(i) => { return i }
            None => {return 0}
        }
    }
    
    /** Get the front element. */
    fn peek(&mut self) -> i32 {
        if  self.out_stack.len() < 1 {
            self.in_stack.reverse();
            mem::swap(&mut self.in_stack, &mut self.out_stack);
        }
        if self.out_stack.len() > 0 {
            return self.out_stack[self.out_stack.len() - 1];
        }
        0
    }
    
    /** Returns whether the queue is empty. */
    fn empty(&self) -> bool {
        if self.in_stack.len() > 0 || self.out_stack.len() > 0 {
            return false;
        }
        true
    }
}

/**
 * Your MyQueue object will be instantiated and called as such:
 * let obj = MyQueue::new();
 * obj.push(x);
 * let ret_2: i32 = obj.pop();
 * let ret_3: i32 = obj.peek();
 * let ret_4: bool = obj.empty();
 */

#[cfg(test)]
mod test {
    use super::MyQueue;

    #[test]
    fn it_works() {
        let mut queue = MyQueue::new();
        queue.push(1);
        queue.push(2);
        assert_eq!(queue.peek(), 1);
        assert_eq!(queue.pop(), 1);
        assert_eq!(queue.empty(), false);
    }
}