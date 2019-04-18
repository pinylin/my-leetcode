/*
 * @lc app=leetcode id=205 lang=rust
 *
 * [205] Isomorphic Strings
 *
 * https://leetcode.com/problems/isomorphic-strings/description/
 *
 * algorithms
 * Easy (36.77%)
 * Total Accepted:    195.8K
 * Total Submissions: 528.7K
 * Testcase Example:  '"egg"\n"add"'
 *
 * Given two strings s and t, determine if they are isomorphic.
 *
 * Two strings are isomorphic if the characters in s can be replaced to get t.
 *
 * All occurrences of a character must be replaced with another character while
 * preserving the order of characters. No two characters may map to the same
 * character but a character may map to itself.
 *
 * Example 1:
 *
 *
 * Input: s = "egg", t = "add"
 * Output: true
 *
 *
 * Example 2:
 *
 *
 * Input: s = "foo", t = "bar"
 * Output: false
 *
 * Example 3:
 *
 *
 * Input: s = "paper", t = "title"
 * Output: true
 *
 * Note:
 * You may assume both s and t have the same length.
 *
 */
use std::collections::HashMap;
use std::collections::HashSet;

impl Solution {
    pub fn is_isomorphic(s: String, t: String) -> bool {
        // best solution
        let mut mappings = HashMap::with_capacity(26);
        let mut chars_s = s.chars();
        let mut chars_t = t.chars();
        loop {
            let char_s = chars_s.next();
            let char_t = chars_t.next();
            if char_s == None {
                // s and t have same length
                break;
            }
            let char_s = char_s.unwrap();
            let char_t = char_t.unwrap();
            let mapping = mappings.entry(char_s).or_insert(char_t);
            if *mapping != char_t {
                return false;
            }
        }

        let mut set = HashSet::with_capacity(26);
        for v in mappings.values() {
            if !set.insert(v) {
                return false;
            }
        }
        true
    }
    pub fn is_isomorphic2(s: String, t: String) -> bool {
        Solution::code(s) == Solution::code(t)
    }
    fn code(s: String) -> String {
        let mut map = HashMap::new();
        let mut start: char = '0';
        let mut res = String::new();
        for ch in s.chars() {
            let v = map.entry(ch).or_insert_with(|| {
                start = ((start as u8) + 1) as char;
                start
            });
            res.push(*v)
        }
        res
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn it_works() {
        assert_eq!(
            Solution::is_isomorphic("add".to_owned(), "egg".to_owned()),
            true
        );
        assert_eq!(
            Solution::is_isomorphic("ad".to_owned(), "aa".to_owned()),
            false
        );
    }
}
