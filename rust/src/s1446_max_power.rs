/*
 * @lc app=leetcode.cn id=1446 lang=rust
 *
 * [1446] 连续字符
 */

// @lc code=start
impl Solution {
    pub fn max_power(s: String) -> i32 {
        let (mut pow, mut count) = (0, 1);
        let chars = s.as_bytes();
        (1..chars.len()).for_each(|i| {
            if chars[i] == chars[i - 1] {
                count += 1;
                pow = std::cmp::max(pow, count);
            } else {
                count = 1;
            }
        });
        std::cmp::max(pow, 1)
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
        assert_eq!(Solution::max_power("hooraaaaaaaaaaay".to_owned()), 11);
        assert_eq!(Solution::max_power("cc".to_owned()), 2)
    }
}
