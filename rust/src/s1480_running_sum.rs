/*
 * @lc app=leetcode.cn id=1480 lang=rust
 *
 * [1480] 一维数组的动态和
 */

// @lc code=start
impl Solution {
    pub fn running_sum(nums: Vec<i32>) -> Vec<i32> {
        // let mut cur = 0;
        // let mut res = vec![0; nums.len()];
        // for idx in 0..nums.len() {
        //     res[idx] = cur + nums[idx];
        //     cur += nums[idx];
        // }
        // res

        nums.iter()
            .scan(0, |acc, x| {
                *acc = *acc + x;
                Some(*acc)
            })
            .collect()
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
        assert_eq!(Solution::running_sum(vec![1, 2, 3, 4]), vec![1, 3, 6, 10])
    }
}
