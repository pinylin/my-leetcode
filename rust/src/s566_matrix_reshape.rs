/*
 * @lc app=leetcode.cn id=566 lang=rust
 *
 * [566] 重塑矩阵
 */

// @lc code=start
impl Solution {
    pub fn matrix_reshape(nums: Vec<Vec<i32>>, r: i32, c: i32) -> Vec<Vec<i32>> {
        if (r * c) as usize != nums[0].len() * nums.len() {
            nums
        } else {
            nums.into_iter()
                .flatten()
                .collect::<Vec<i32>>()
                .chunks(c as usize)
                .map(|x| x.to_vec())
                .collect()
        }
    }
}
// @lc code=end

pub struct Solution {}
