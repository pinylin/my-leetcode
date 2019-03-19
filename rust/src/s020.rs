/*
 * @lc app=leetcode id=20 lang=rust
 *
 * [20] Valid Parentheses
 *
 * https://leetcode.com/problems/valid-parentheses/description/
 *
 * algorithms
 * Easy (35.95%)
 * Total Accepted:    525.4K
 * Total Submissions: 1.5M
 * Testcase Example:  '"()"'
 *
 * Given a string containing just the characters '(', ')', '{', '}', '[' and
 * ']', determine if the input string is valid.
 *
 * An input string is valid if:
 *
 *
 * Open brackets must be closed by the same type of brackets.
 * Open brackets must be closed in the correct order.
 *
 *
 * Note that an empty string is also considered valid.
 *
 * Example 1:
 *
 *
 * Input: "()"
 * Output: true
 *
 *
 * Example 2:
 *
 *
 * Input: "()[]{}"
 * Output: true
 *
 *
 * Example 3:
 *
 *
 * Input: "(]"
 * Output: false
 *
 *
 * Example 4:
 *
 *
 * Input: "([)]"
 * Output: false
 *
 *
 * Example 5:
 *
 *
 * Input: "{[]}"
 * Output: true
 *
 *
 */
impl Solution {
    pub fn is_valid(s: String) -> bool {
        fn pair(arg: u8) -> u8 {
            match arg {
                b'(' => b')',
                b'[' => b']',
                _ => b'}',
            }
        }
        let mut stack = vec![];
        for c in s.bytes() {
            match c {
                b'(' | b'[' | b'{' => stack.push(pair(c)),
                _ => {
                    let pop = stack.pop().unwrap_or(0);
                    if pop != c {
                        return false;
                    }
                }
            }
        }
        stack.is_empty()
    }
}

pub struct Solution;

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn it_works() {
        assert_eq!(Solution::is_valid("()".into()), true);
        assert_eq!(Solution::is_valid("(){}[]".into()), true);
        assert_eq!(Solution::is_valid("(}".into()), false);
        assert_eq!(Solution::is_valid("([)]".into()), false);
        assert_eq!(Solution::is_valid("{[]}".into()), true);
    }
}
