/*
 * @lc app=leetcode.cn id=1556 lang=rust
 *
 * [1556] 千位分隔数
 */

// @lc code=start
impl Solution {
    pub fn thousand_separator(n: i32) -> String {
        let s = n.to_string();
        let v = s.split("").filter(|&c| c != "").collect::<Vec<_>>();
        let v = v.rchunks(3).rev().map(|c| c.concat()).collect::<Vec<_>>();

        v.join(".")
    }
}
// @lc code=end

pub struct Solution;
#[cfg(test)]
mod test {
    use super::Solution;
    // use crate::utils::vec2d;
    #[test]
    fn it_works() {
        assert_eq!(Solution::thousand_separator(3893), "3.893".to_owned());
    }
}
