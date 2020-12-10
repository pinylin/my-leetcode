/*
 * @lc app=leetcode.cn id=1332 lang=rust
 *
 * [1332] 删除回文子序列
 */

// @lc code=start
impl Solution {
    pub fn remove_palindrome_sub(s: String) -> i32 {
        // 认真看题目，子序列
        if s.is_empty() {
            0
        } else if s == s.chars().rev().collect::<String>() {
            1
        } else {
            2
        }
    }
}
// @lc code=end

pub struct Solution;

#[cfg(test)]
mod test {
    use super::Solution;
    #[test]
    fn it_works() {
        assert_eq!(Solution::remove_palindrome_sub("ababa".to_owned()), 1);
        assert_eq!(Solution::remove_palindrome_sub("baabb".to_owned()), 2);
        assert_eq!(
            Solution::remove_palindrome_sub("abaabbaaabbbaaaabbbb".to_owned()),
            2
        );
    }
}
