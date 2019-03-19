/*
 * @lc app=leetcode id=53 lang=rust
 *
 * [53] Maximum Subarray
 *
 * https://leetcode.com/problems/maximum-subarray/description/
 *
 * algorithms
 * Easy (42.86%)
 * Total Accepted:    476.2K
 * Total Submissions: 1.1M
 * Testcase Example:  '[-2,1,-3,4,-1,2,1,-5,4]'
 *
 * Given an integer array nums, find the contiguous subarray (containing at
 * least one number) which has the largest sum and return its sum.
 * 
 * Example:
 * 
 * 
 * Input: [-2,1,-3,4,-1,2,1,-5,4],
 * Output: 6
 * Explanation: [4,-1,2,1] has the largest sum = 6.
 * 
 * 
 * Follow up:
 * 
 * If you have figured out the O(n) solution, try coding another solution using
 * the divide and conquer approach, which is more subtle.
 * 
 */
impl Solution {
    pub fn max_sub_array(nums: Vec<i32>) -> i32 {
        let (mut sum, mut max) = (nums[0], nums[0]);
        for i in &nums[1..] {
            // if sum < 0 那么加上后面只会更小， 故置零
            if sum < 0 {
                sum = 0;
            }
            sum += i;
            max = max.max(sum);
        }
        max
    }
}

pub struct Solution;

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn it_works() {
        assert_eq!(Solution::max_sub_array(vec![-2, 1, -3, 4, -1, 2, 1, -5, 4]),6);
        assert_eq!(Solution::max_sub_array(vec![-2, 1]), 1);
        assert_eq!(Solution::max_sub_array(vec![-2]), -2);
        assert_eq!(Solution::max_sub_array(vec![-2, -1, -3]), -1);
    }
}