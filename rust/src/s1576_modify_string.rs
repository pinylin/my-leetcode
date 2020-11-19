/*
 * @lc app=leetcode.cn id=1576 lang=rust
 *
 * [1576] 替换所有的问号
 */

// @lc code=start
impl Solution {
    pub fn modify_string(s: String) -> String {
        let mut res: Vec<char> = s.chars().collect();
        (0..s.len()).for_each(|i| {
            if res[i] == '?' {
                let left = if i == 0 { None } else { Some(res[i - 1]) };
                let right = if i == s.len() - 1 {
                    None
                } else {
                    Some(res[i + 1])
                };
                res[i] = ('a'..='z')
                    // .into_iter()
                    .find(|&x| Some(x) != left && Some(x) != right)
                    .unwrap();
            }
        });
        res.into_iter().collect()
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
            Solution::modify_string("?abc".to_owned()),
            "babc".to_owned()
        );
    }
}
