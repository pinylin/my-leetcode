/*
 * @lc app=leetcode.cn id=1544 lang=rust
 *
 * [1544] 整理字符串
 */

// @lc code=start
impl Solution {
    pub fn make_good(s: String) -> String {
        let mut stack: Vec<char> = Vec::new();

        for c in s.chars() {
            if stack.len() > 0 && (c as i32 - stack[stack.len() - 1] as i32).abs() == 32 {
                stack.pop();
            } else {
                stack.push(c);
            }
        }

        return stack.iter().collect();
    }
}
// @lc code=end

pub struct Solution;
#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn it_works() {
        assert_eq!(
            Solution::make_good("leEeetcode".to_owned()),
            "leetcode".to_owned()
        );
    }
}
