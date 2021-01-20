/*
 * @lc app=leetcode.cn id=1051 lang=rust
 *
 * [1051] 高度检查器
 */

// @lc code=start
impl Solution {
    pub fn height_checker(heights: Vec<i32>) -> i32 {
        let mut band = vec![0; 101];
        for h in heights.iter() {
            band[*h as usize] += 1;
        }
        let mut count = 0;
        let mut j = 0;
        for i in 1..band.len() {
            while band[i] > 0 {
                band[i] -= 1;
                if heights[j] != i as i32 {
                    count += 1;
                }
                j += 1;
            }
        }

        count
    }
}
// @lc code=end

pub struct Solution {}
