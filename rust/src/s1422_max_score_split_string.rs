/*
 * @lc app=leetcode.cn id=1422 lang=rust
 *
 * [1422] 分割字符串的最大得分
 */

// @lc code=start
impl Solution {
    pub fn max_score(s: String) -> i32 {
        let mut res = 0;
        let chars: Vec<char> = s.chars().collect();
        (1..chars.len()).for_each(|p| {
            let mut tmp = 0;
            for i in 0..p {
                if chars[i] == '0' {
                    tmp += 1;
                }
            }
            for j in p..chars.len() {
                if chars[j] == '1' {
                    tmp += 1;
                }
            }
            res = std::cmp::max(res, tmp);
        });

        res
    }
}
// @lc code=end
pub struct Solution;
#[cfg(test)]
mod test {
    use super::Solution;
    // use crate::utils::vec2d;
    #[test]
    fn it_works() {
        assert_eq!(Solution::max_score("00111".to_owned()), 5);
        assert_eq!(Solution::max_score("1111".to_owned()), 3)
    }
}
