/*
 * @lc app=leetcode.cn id=1614 lang=rust
 *
 * [1614] 括号的最大嵌套深度
 */

// @lc code=start
impl Solution {
    pub fn max_depth(s: String) -> i32 {
        let mut depth = 0;
        let mut vec = Vec::new();
        for item in s.chars() {
            match item {
                '(' => vec.push("("),
                ')' => {
                    vec.pop();
                }
                _ => {}
            }
            depth = depth.max(vec.len());
        }
        depth as i32
    }
}
// @lc code=end

pub struct Solution;
