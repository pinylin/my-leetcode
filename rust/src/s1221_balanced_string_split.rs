/*
 * @lc app=leetcode.cn id=1221 lang=rust
 *
 * [1221] 分割平衡字符串
 */

// @lc code=start
impl Solution {
    pub fn balanced_string_split(s: String) -> i32 {
        let mut count = 0;
        s.chars().fold(0, |mut res, x| {
            match x {
                'L' => count += 1,
                'R' => count -= 1,
                _ => {}
            }
            if count == 0 {
                res += 1;
            }
            res
        })

        // let (mut res, mut count) = (0, 0);
        // for ch in s.chars() {
        //     match ch {
        //         'L' => count += 1,
        //         'R' => count -= 1,
        //         _ => {}
        //     }
        //     if count == 0 {
        //         res += 1;
        //     }
        // }
        // res
    }
}
// @lc code=end

pub struct Solution;
#[cfg(test)]
mod test {
    use super::Solution;
    #[test]
    fn it_works() {
        assert_eq!(Solution::balanced_string_split("RLLLLRRRLR".to_owned()), 3);
        assert_eq!(Solution::balanced_string_split("RLRRLLRLRL".to_owned()), 4);
    }
}
