/*
 * @lc app=leetcode id=290 lang=rust
 *
 * [290] Word Pattern
 */
impl Solution {
    pub fn word_pattern(pattern: String, str: String) -> bool {
        use std::collections::HashSet;
        if pattern.is_empty() {
            return str.is_empty();
        }

        let words: Vec<String> = str
            .split(' ')
            .filter(|x| !x.is_empty())
            .map(|x| x.to_string())
            .collect();
        if words.len() != pattern.len() {
            return false;
        }
        let mut set = HashSet::new();
        let mut word_set = HashSet::new();
        let mut p_set = HashSet::new();
        for (i, ch) in pattern.chars().enumerate() {
            p_set.insert(ch);
            word_set.insert(format!("{}", words[i]));
            set.insert(format!("{}{}", ch, words[i]));
        }
        if p_set.len() == word_set.len() && p_set.len() == set.len() {
            return true;
        }
        false
    }
}

pub struct Solution;

#[cfg(test)]

mod test {
    use super::Solution;
    #[test]
    fn it_works() {
        assert_eq!(
            Solution::word_pattern("abba".to_string(), "dog   cat   cat   dog".to_string()),
            true
        );
        assert_eq!(
            Solution::word_pattern("abba".to_string(), "dog cat cat dog".to_string()),
            true
        );
        assert_eq!(
            Solution::word_pattern("abba".to_string(), "dog cat cat fish".to_string()),
            false
        );
        assert_eq!(
            Solution::word_pattern("aaaa".to_string(), "dog cat cat dog".to_string()),
            false
        );
        assert_eq!(
            Solution::word_pattern("abba".to_string(), "dog dog dog dog".to_string()),
            false
        );
    }
}
