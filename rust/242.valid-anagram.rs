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
        if s.len() != t.len() {
            return false;
        }
        let mut res = 0;
        for ch in s.chars() {
            res = res ^ ch as u8;
        }
        if res == 0 {
            if  s.chars().map(|c| c as u8).sum::<u8>() ==  t.chars().map(|c| c as u8).sum::<u8>() {
                return true;
        }
        }
        for ch in t.chars() {
            res = res ^ ch as u8;
        }
        if res == 0 {
            return true;
        }

        false
    }
}

