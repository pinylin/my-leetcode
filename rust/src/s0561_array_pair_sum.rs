/*
 * @lc app=leetcode.cn id=561 lang=rust
 *
 * [561] 数组拆分 I
 */

// @lc code=start
impl Solution {
    pub fn array_pair_sum(mut nums: Vec<i32>) -> i32 {
        nums.sort_unstable();
        nums.chunks(2).fold(0, |res, x| res + x[0])
    }
}
// @lc code=end

pub struct Solution {}
