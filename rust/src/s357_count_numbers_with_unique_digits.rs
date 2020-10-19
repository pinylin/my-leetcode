/*
 * @lc app=leetcode.cn id=357 lang=rust
 *
 * [357] 计算各个位数不同的数字个数
 */

// @lc code=start
use std::cmp::Ordering;
impl Solution {
    pub fn count_numbers_with_unique_digits(n: i32) -> i32 {
        if n == 0 {return 1};
        let mut res = 10;
        let mut cnt = 9;
        for i in 2..=n {
            cnt *= (11 - i);
            res += cnt;
        }
        res
    }
}
// @lc code=end

