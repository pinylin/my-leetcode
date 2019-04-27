/*
 * @lc app=leetcode id=225 lang=rust
 *
 * [225] Implement Stack using Queues
 *
 * https://leetcode.com/problems/implement-stack-using-queues/description/
 *
 * algorithms
 * Easy (38.08%)
 * Total Accepted:    125.1K
 * Total Submissions: 323.3K
 * Testcase Example:  '["MyStack","push","push","top","pop","empty"]\n[[],[1],[2],[],[],[]]'
 *
 * Implement the following operations of a stack using queues.
 *
 *
 * push(x) -- Push element x onto stack.
 * pop() -- Removes the element on top of the stack.
 * top() -- Get the top element.
 * empty() -- Return whether the stack is empty.
 *
 *
 * Example:
 *
 *
 * MyStack stack = new MyStack();
 *
 * stack.push(1);
 * stack.push(2);
 * stack.top();   // returns 2
 * stack.pop();   // returns 2
 * stack.empty(); // returns false
 *
 * Notes:
 *
 *
 * You must use only standard operations of a queue -- which means only push to
 * back, peek/pop from front, size, and is empty operations are valid.
 * Depending on your language, queue may not be supported natively. You may
 * simulate a queue by using a list or deque (double-ended queue), as long as
 * you use only standard operations of a queue.
 * You may assume that all operations are valid (for example, no pop or top
 * operations will be called on an empty stack).
 *
 *
 */
struct MyStack {
    stack: Vec<i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
#[allow(dead_code)]
impl MyStack {
    /** Initialize your data structure here. */
    fn new() -> Self {
        MyStack { stack: Vec::new() }
    }

    /** Push element x onto stack. */
    fn push(&mut self, x: i32) {
        self.stack.push(x);
    }

    /** Removes the element on top of the stack and returns that element. */
    fn pop(&mut self) -> i32 {
        match self.stack.pop() {
            Some(i) => i,
            None => 0,
        }
    }

    /** Get the top element. */
    fn top(&self) -> i32 {
        if !self.stack.is_empty() {
            return self.stack[self.stack.len() - 1];
        }
        0
    }

    /** Returns whether the stack is empty. */
    fn empty(&self) -> bool {
        if !self.stack.is_empty() {
            return false;
        }
        true
    }
}

/**
 * Your MyStack object will be instantiated and called as such:
 * let obj = MyStack::new();
 * obj.push(x);
 * let ret_2: i32 = obj.pop();
 * let ret_3: i32 = obj.top();
 * let ret_4: bool = obj.empty();
 */

#[cfg(test)]
mod test {
    use super::MyStack;

    #[test]
    fn it_works() {
        let mut my_stack = MyStack::new();
        my_stack.push(-2);
        my_stack.push(0);
        my_stack.push(-3);
        assert_eq!(my_stack.top(), -3);
        my_stack.pop();
        assert_eq!(my_stack.top(), 0);
        assert_eq!(my_stack.empty(), false);
    }
}
