/*
 * @lc app=leetcode id=125 lang=rust
 *
 * [125] Valid Palindrome
 *
 * https://leetcode.com/problems/valid-palindrome/description/
 *
 * algorithms
 * Easy (30.29%)
 * Total Accepted:    338K
 * Total Submissions: 1.1M
 * Testcase Example:  '"A man, a plan, a canal: Panama"'
 *
 * Given a string, determine if it is a palindrome, considering only
 * alphanumeric characters and ignoring cases.
 * 
 * Note: For the purpose of this problem, we define empty string as valid
 * palindrome.
 * 
 * Example 1:
 * 
 * 
 * Input: "A man, a plan, a canal: Panama"
 * Output: true
 * 
 * 
 * Example 2:
 * 
 * 
 * Input: "race a car"
 * Output: false
 * 
 * 
 */
impl Solution {
    pub fn is_palindrome(s: String) -> bool {
        if s.is_empty() {
            return true
        }
        let chs = s.as_bytes();
        let (mut i, mut j) = (0usize, chs.len() - 1);
        while i < j {
            let l = chs[i] as char;
            let r = chs[j] as char;
            if !l.is_alphanumeric() {
                i += 1;
                continue;
            }

            if !r.is_alphanumeric() {
                j -= 1;
                continue;
            }

            if l.to_uppercase().to_string() != r.to_uppercase().to_string() {
                return false;
            } else {
                i += 1;
                j -= 1;
            }
        }
        true
    }
}

