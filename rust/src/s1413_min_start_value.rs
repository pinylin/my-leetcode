/*
 * @lc app=leetcode.cn id=1413 lang=rust
 *
 * [1413] 逐步求和得到正数的最小值
 */

// @lc code=start
impl Solution {
    pub fn min_start_value(nums: Vec<i32>) -> i32 {
        let mut min = 1;
        let mut sum = 0;
        for num in nums.iter() {
            sum += num;
            if sum <= 0 && sum < min {
                min = sum;
            }
        }
        if min < 1 {
            return 1 - min;
        }
        1
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
        assert_eq!(Solution::min_start_value(vec![-3, 2, -3, 4, 2]), 5);
        assert_eq!(Solution::min_start_value(vec![1, -2, -3]), 5);
        assert_eq!(Solution::min_start_value(vec![5, 4, -5, -5, 0]), 2);
    }
}
