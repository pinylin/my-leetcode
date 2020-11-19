/*
 * @lc app=leetcode.cn id=1550 lang=rust
 *
 * [1550] 存在连续三个奇数的数组
 */

// @lc code=start
impl Solution {
    pub fn three_consecutive_odds(arr: Vec<i32>) -> bool {
        let mut acc = 0;
        for item in arr.iter() {
            match item & 1 {
                0 => acc = 0,
                _ => acc += 1,
            }
            if acc == 3 {
                return true;
            };
        }

        false
    }
}
// @lc code=end

pub struct Solution;
#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn it_works() {
        assert_eq!(Solution::three_consecutive_odds(vec![5, 4, 3, 2, 1]), false);
    }
}
