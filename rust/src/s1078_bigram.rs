/*
 * @lc app=leetcode.cn id=1078 lang=rust
 *
 * [1078] Bigram 分词
 */

// @lc code=start
impl Solution {
    pub fn find_ocurrences(text: String, first: String, second: String) -> Vec<String> {
        let mut res = vec![];
        text.split_whitespace()
            .collect::<Vec<_>>()
            .windows(3)
            .for_each(|v| {
                if v[0] == first && v[1] == second {
                    res.push(v[2].to_owned())
                }
            });

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
            Solution::find_ocurrences(
                "alice is a good girl she is a good student".to_owned(),
                "a".to_owned(),
                "good".to_owned()
            ),
            vec_string!["girl", "student"]
        );
    }
}
