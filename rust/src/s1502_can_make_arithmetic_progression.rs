/*
 * @lc app=leetcode.cn id=1502 lang=rust
 *
 * [1502] 判断能否形成等差数列
 */

// @lc code=start
impl Solution {
    pub fn can_make_arithmetic_progression(arr: Vec<i32>) -> bool {
        if arr.len() > 2 {
            let mut arr = arr;
            arr.sort();
            let diff = arr[1] - arr[0];
            for idx in 1..arr.len() {
                if arr[idx] - arr[idx - 1] != diff {
                    return false;
                }
            }
        }
        true
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
        assert_eq!(
            Solution::can_make_arithmetic_progression(vec![3, 5, 1]),
            true
        );
    }
}
