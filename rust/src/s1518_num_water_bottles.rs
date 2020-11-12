/*
 * @lc app=leetcode.cn id=1518 lang=rust
 *
 * [1518] 换酒问题
 */

// @lc code=start
impl Solution {
    pub fn num_water_bottles(num_bottles: i32, num_exchange: i32) -> i32 {
        let mut res = num_bottles;
        let mut remain = num_bottles;
        while remain >= num_exchange {
            let term = remain / num_exchange;
            res += term;
            remain = remain % num_exchange + term;
        }
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
        assert_eq!(Solution::num_water_bottles(15, 4), 19);
    }
}
