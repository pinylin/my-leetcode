/*
 * @lc app=leetcode.cn id=1408 lang=rust
 *
 * [1408] 数组中的字符串匹配
 */

// @lc code=start
use std::collections::HashSet;
impl Solution {
    pub fn string_matching(words: Vec<String>) -> Vec<String> {
        let mut set: HashSet<String> = HashSet::new();
        let mut words = words.to_vec();
        words.sort_unstable_by(|a, b| a.len().cmp(&b.len()));
        for (i, w) in words.iter().enumerate() {
            for j in i + 1..words.len() {
                if words[j].contains(w) {
                    set.insert(w.clone());
                }
            }
        }
        set.into_iter().collect::<Vec<String>>()
    }
}
// @lc code=end

pub struct Solution;
#[cfg(test)]
mod test {
    use super::Solution;
    use crate::vec_string;
    // use crate::utils::vec2d;
    #[test]
    fn it_works() {
        assert_eq!(
            Solution::string_matching(vec_string!["blue", "green", "bu"]),
            vec![""; 0]
        );
        assert_eq!(
            Solution::string_matching(vec_string!["mass", "as", "hero", "superhero"]),
            vec_string!["as", "hero"]
        );
    }
}
