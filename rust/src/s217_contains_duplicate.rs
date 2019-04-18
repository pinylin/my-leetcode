/*
 * @lc app=leetcode id=217 lang=rust
 *
 * [217] Contains Duplicate
 *
 * https://leetcode.com/problems/contains-duplicate/description/
 *
 * algorithms
 * Easy (50.89%)
 * Total Accepted:    321.2K
 * Total Submissions: 624.2K
 * Testcase Example:  '[1,2,3,1]'
 *
 * Given an array of integers, find if the array contains any duplicates.
 *
 * Your function should return true if any value appears at least twice in the
 * array, and it should return false if every element is distinct.
 *
 * Example 1:
 *
 *
 * Input: [1,2,3,1]
 * Output: true
 *
 * Example 2:
 *
 *
 * Input: [1,2,3,4]
 * Output: false
 *
 * Example 3:
 *
 *
 * Input: [1,1,1,3,3,4,3,2,4,2]
 * Output: true
 *
 */
use std::collections::HashSet;

impl Solution {
    // best solution
    pub fn contains_duplicate(mut nums: Vec<i32>) -> bool {
        if nums.is_empty() {
            return false;
        }

        nums.sort_unstable();

        for i in 0..nums.len() - 1 {
            if nums[i] == nums[i + 1] {
                return true;
            }
        }

        return false;
    }

    pub fn contains_duplicate_2(nums: Vec<i32>) -> bool {
        let mut set = HashSet::new();
        for v in nums.iter() {
            if !set.insert(v) {
                return true;
            }
        }
        false
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn it_works() {
        assert_eq!(Solution::contains_duplicate(vec![1, 1, 3, 4, 5]), true);
        assert_eq!(Solution::contains_duplicate(vec![4, 0, 8]), false);
    }
}
