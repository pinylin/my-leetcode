/*
 * @lc app=leetcode.cn id=1523 lang=rust
 *
 * [1523] 在区间范围内统计奇数数目
 */

// @lc code=start
impl Solution {
    pub fn count_odds(low: i32, high: i32) -> i32 {
        let mut res = 0;

        if low % 2 == 1 || high % 2 == 1 {
            res = 1;
        }
        res + (high - low) / 2
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
        assert_eq!(Solution::count_odds(3, 7), 3);
    }
}
