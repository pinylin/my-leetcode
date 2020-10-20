/*
 * @lc app=leetcode.cn id=1608 lang=rust
 *
 * [1608] 特殊数组的特征值
 */

// @lc code=start
impl Solution {
    pub fn special_array(nums: Vec<i32>) -> i32 {
        for i in 0..=nums.len() as i32 {
            let mut res = 0;
            for item in nums.iter() {
                if item >= &i {
                    res += 1;
                }
            }
            if res == i {
                return res;
            }
        }
        -1
    }
}
// @lc code=end

pub struct Solution;
