/*
 * @lc app=leetcode.cn id=1464 lang=rust
 *
 * [1464] 数组中两元素的最大乘积
 */

// @lc code=start
impl Solution {
    pub fn max_product(mut nums: Vec<i32>) -> i32 {
        nums.sort();
        let len = nums.len();
        (nums[len - 1] - 1) * (nums[len - 2] - 1)
    }
}
// @lc code=end
pub struct Solution;
#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn it_works() {
        assert_eq!(Solution::max_product(vec![1, 5, 4, 5]), 16)
    }
}
