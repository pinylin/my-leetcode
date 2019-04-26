/*
 * @lc app=leetcode id=242 lang=rust
 *
 * [242] Valid Anagram
 *
 * https://leetcode.com/problems/valid-anagram/description/
 *
 * algorithms
 * Easy (51.02%)
 * Total Accepted:    324K
 * Total Submissions: 627.1K
 * Testcase Example:  '"anagram"\n"nagaram"'
 *
 * Given two strings s and t , write a function to determine if t is an anagram
 * of s.
 *
 * Example 1:
 *
 *
 * Input: s = "anagram", t = "nagaram"
 * Output: true
 *
 *
 * Example 2:
 *
 *
 * Input: s = "rat", t = "car"
 * Output: false
 *
 *
 * Note:
 * You may assume the string contains only lowercase alphabets.
 *
 * Follow up:
 * What if the inputs contain unicode characters? How would you adapt your
 * solution to such case?
 *
 */
impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool {
        let mut band = vec![0i16; 26];
        for ch in s.chars() {
            band[ch as usize - 97] += 1;
        }
        for ch in t.chars() {
            band[ch as usize - 97] -= 1;
        }
        for item in band.iter() {
            if *item != 0i16 {
                return false;
            }
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
        assert_eq!(
            Solution::is_anagram("aa".to_owned(), "bb".to_owned()),
            false
        );
        assert_eq!(
            Solution::is_anagram("xaaddy".to_owned(), "xbbccy".to_owned()),
            false
        );
    }
}
