/*
 * @lc app=leetcode.cn id=1486 lang=rust
 *
 * [1486] 数组异或操作
 */

// @lc code=start
impl Solution {
    pub fn xor_operation(n: i32, start: i32) -> i32 {
        let mut res = start;
        for i in 1..n {
            res ^= start + i * 2;
        }
        res
    }
}
// @lc code=end
pub struct Solution;
#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn it_works() {
        assert_eq!(Solution::xor_operation(4, 3), 8)
    }
}
