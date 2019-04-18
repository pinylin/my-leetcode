/*
 * @lc app=leetcode id=136 lang=rust
 *
 * [136] Single Number
 *
 * https://leetcode.com/problems/single-number/description/
 *
 * algorithms
 * Easy (59.11%)
 * Total Accepted:    439.5K
 * Total Submissions: 738K
 * Testcase Example:  '[2,2,1]'
 *
 * Given a non-empty array of integers, every element appears twice except for
 * one. Find that single one.
 *
 * Note:
 *
 * Your algorithm should have a linear runtime complexity. Could you implement
 * it without using extra memory?
 *
 * Example 1:
 *
 *
 * Input: [2,2,1]
 * Output: 1
 *
 *
 * Example 2:
 *
 *
 * Input: [4,1,2,1,2]
 * Output: 4
 *
 *
 */
impl Solution {
    pub fn single_number(nums: Vec<i32>) -> i32 {
        let mut res = 0;
        for i in 0..nums.len() {
            res = res ^ nums[i];
        }
        res
    }
    // pub fn single_number(nums: &Vec<i32>) -> i32 {
    //     nums.iter().fold(0, |acc, x| acc ^ x)
    // }
}

pub struct Solution;

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn it_works() {
        assert_eq!(Solution::single_number(vec![1, 1, 2, 2, 4]), 4);
    }
}
