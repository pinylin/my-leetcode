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
        // let chs = s.chars().collect::<Vec<_>>();
        let chs = s.as_bytes();
        let (mut i, mut j) = (0usize, chs.len() - 1);
        while i < j {
            while i < chs.len() && !chs[i].is_ascii_alphanumeric(){ i += 1; }
            while j > 0 && !chs[j].is_ascii_alphanumeric() { j -= 1; }
            if i >= j { break }
            if chs[i].to_ascii_lowercase() != chs[j].to_ascii_lowercase() {
                return false
            }
            i += 1; j -= 1;
        }
        true
    }
}


pub struct Solution;

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn it_works() {
        assert_eq!(Solution::is_palindrome("A man, a plan, a canal: Panama".to_owned()), true);
        assert_eq!(Solution::is_palindrome("race a car".to_owned()), false);
        assert_eq!(Solution::is_palindrome("0P".to_owned()), false);
    }
}