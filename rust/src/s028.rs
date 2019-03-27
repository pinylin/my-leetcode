/*
 * @lc app=leetcode id=28 lang=rust
 *
 * [28] Implement strStr()
 *
 * https://leetcode.com/problems/implement-strstr/description/
 *
 * algorithms
 * Easy (31.30%)
 * Total Accepted:    390.2K
 * Total Submissions: 1.2M
 * Testcase Example:  '"hello"\n"ll"'
 *
 * Implement strStr().
 *
 * Return the index of the first occurrence of needle in haystack, or -1 if
 * needle is not part of haystack.
 *
 * Example 1:
 *
 *
 * Input: haystack = "hello", needle = "ll"
 * Output: 2
 *
 *
 * Example 2:
 *
 *
 * Input: haystack = "aaaaa", needle = "bba"
 * Output: -1
 *
 *
 * Clarification:
 *
 * What should we return when needle is an empty string? This is a great
 * question to ask during an interview.
 *
 * For the purpose of this problem, we will return 0 when needle is an empty
 * string. This is consistent to C's strstr() and Java's indexOf().
 *
 */
impl Solution {
    pub fn str_str(haystack: String, needle: String) -> i32 {
        haystack.find(&needle).map(|n| n as i32).unwrap_or(-1)
    }
}

pub struct Solution;

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn it_works() {
        assert_eq!(Solution::str_str("hello".into(), "ll".into()), 2);
        assert_eq!(Solution::str_str("aaaaa".into(), "bba".into()), -1);
        assert_eq!(Solution::str_str("hello".into(), "".into()), 0);
        assert_eq!(Solution::str_str("a".into(), "a".into()), 0);
        assert_eq!(Solution::str_str("mississippi".into(), "pi".into()), 9);
        assert_eq!(Solution::str_str("".into(), "a".into()), -1);
    }
}
