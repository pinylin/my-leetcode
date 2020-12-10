/*
 * @lc app=leetcode.cn id=1323 lang=rust
 *
 * [1323] 6 和 9 组成的最大数字
 */

// @lc code=start
impl Solution {
    pub fn maximum69_number(num: i32) -> i32 {
        let s = num.to_string();
        s.replacen('6', "9", 1).parse().unwrap()
    }
}
// @lc code=end

pub struct Solution;

#[cfg(test)]
mod test {
    use super::Solution;
    #[test]
    fn it_works() {
        assert_eq!(Solution::maximum69_number(96699), 99699);
        assert_eq!(Solution::maximum69_number(999999), 999999);
    }
}
