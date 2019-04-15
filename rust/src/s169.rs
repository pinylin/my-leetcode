/*
 * @lc app=leetcode id=169 lang=rust
 *
 * [169] Majority Element
 *
 * https://leetcode.com/problems/majority-element/description/
 *
 * algorithms
 * Easy (51.59%)
 * Total Accepted:    366.7K
 * Total Submissions: 704K
 * Testcase Example:  '[3,2,3]'
 *
 * Given an array of size n, find the majority element. The majority element is
 * the element that appears more than ⌊ n/2 ⌋ times.
 * 
 * You may assume that the array is non-empty and the majority element always
 * exist in the array.
 * 
 * Example 1:
 * 
 * 
 * Input: [3,2,3]
 * Output: 3
 * 
 * Example 2:
 * 
 * 
 * Input: [2,2,1,1,1,2,2]
 * Output: 2
 * 
 * 
 */
impl Solution {
    // 好厉害 我只能想到hashmap
    pub fn majority_element(nums: Vec<i32>) -> i32 {
        let mut count = 0;
        let mut candidate = nums[0];
        for num in nums.iter() {
            if count == 0 {
                candidate = *num;
            }
            count += if *num == candidate { 1 } else { -1 };
        }
        candidate
    }
}

pub struct Solution;

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn it_works() {
        assert_eq!(Solution::majority_element(vec![2,2,1,1,1,2,2]), 2);
        assert_eq!(Solution::majority_element(vec![1,2,3]), 3);
    }
}