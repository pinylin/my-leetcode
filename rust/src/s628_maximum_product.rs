/*
 * @lc app=leetcode.cn id=628 lang=rust
 *
 * [628] 三个数的最大乘积
 */

// @lc code=start
impl Solution {
    pub fn maximum_product(mut nums: Vec<i32>) -> i32 {
        nums.sort_unstable();
        let len = nums.len();
        std::cmp::max(nums[0] * nums[1] * nums[len-1], nums[len-3] * nums[len-2] * nums[len-1])
    }
}
// @lc code=end

pub struct Solution{}
