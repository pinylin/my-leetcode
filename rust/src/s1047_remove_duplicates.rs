/*
 * @lc app=leetcode.cn id=1047 lang=rust
 *
 * [1047] 删除字符串中的所有相邻重复项
 */

// @lc code=start
impl Solution {
    pub fn remove_duplicates(s: String) -> String {
        let mut stack = Vec::new();
        for c in s.chars() {
            if stack.len() > 0 && stack[stack.len() - 1] == c {
                stack.pop();
            } else {
                stack.push(c);
            }
        }
        stack.iter().collect()
    }
}
// @lc code=end

pub struct Solution {}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn it_works() {
        assert_eq!(
            Solution::remove_duplicates("abbaca".to_owned()),
            "ca".to_owned()
        );
    }
}
