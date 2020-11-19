/*
 * @lc app=leetcode.cn id=1455 lang=rust
 *
 * [1455] 检查单词是否为句中其他单词的前缀
 */

// @lc code=start
impl Solution {
    pub fn is_prefix_of_word(sentence: String, search_word: String) -> i32 {
        if let Some((i, _)) = sentence
            .split_ascii_whitespace()
            .enumerate()
            .find(|&(_, word)| word.starts_with(&search_word))
        {
            i as i32 + 1
        } else {
            -1
        }
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
            Solution::is_prefix_of_word(
                "this problem is an easy problem".to_owned(),
                "pro".to_owned()
            ),
            2
        )
    }
}
