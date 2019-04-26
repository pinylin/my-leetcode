/*
 * @lc app=leetcode id=303 lang=rust
 *
 * [303] Range Sum Query - Immutable
 */
struct NumArray {
    sums: Vec<i32>
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl NumArray {

    fn new(nums: Vec<i32>) -> Self {
        let mut sums = vec![0i32; nums.len() + 1];
        for (i, n) in nums.iter().enumerate() {
            sums[i + 1] = sums[i] + *n;
        }
        NumArray { sums: sums }
    }
    
    fn sum_range(&self, i: i32, j: i32) -> i32 {
        self.sums[(j + 1) as usize] - self.sums[i as usize]
    }
}

/**
 * Your NumArray object will be instantiated and called as such:
 * let obj = NumArray::new(nums);
 * let ret_1: i32 = obj.sum_range(i, j);
 */

