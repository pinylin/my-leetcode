/*
 * @lc app=leetcode.cn id=1160 lang=rust
 *
 * [1160] 拼写单词
 */

// @lc code=start
impl Solution {
    pub fn count_characters(words: Vec<String>, chars: String) -> i32 {
        let mut res = 0;
        let mut letter_bag = vec![0; 26];
        for b in chars.bytes() {
            letter_bag[b as usize - 97] += 1;
        }
        for word in words {
            let mut dic = vec![0; 26];
            let mut learned = true;
            for b in word.bytes() {
                dic[b as usize - 97] += 1;
                if dic[b as usize - 97] > letter_bag[b as usize - 97] {
                    learned = false;
                    break;
                }
            }
            if learned {
                res += word.len() as i32
            }
        }
        res
    }
}
// @lc code=end

pub struct Solution;
#[cfg(test)]
mod test {
    use super::Solution;
    use crate::vec_string;
    #[test]
    fn it_works() {
        assert_eq!(
            Solution::count_characters(vec_string!["cat", "bt", "hat", "tree"], "atach".to_owned()),
            6
        );
        assert_eq!(
            Solution::count_characters(
                vec_string!["hello", "world", "leetcode"],
                "welldonehoneyr".to_owned()
            ),
            10
        );
    }
}
