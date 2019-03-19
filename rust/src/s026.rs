/*
 * @lc app=leetcode id=26 lang=rust
 *
 * [26] Remove Duplicates from Sorted Array
 *
 * https://leetcode.com/problems/remove-duplicates-from-sorted-array/description/
 *
 * algorithms
 * Easy (39.74%)
 * Total Accepted:    536.7K
 * Total Submissions: 1.3M
 * Testcase Example:  '[1,1,2]'
 *
 * Given a sorted array nums, remove the duplicates in-place such that each
 * element appear only once and return the new length.
 * 
 * Do not allocate extra space for another array, you must do this by modifying
 * the input array in-place with O(1) extra memory.
 * 
 * Example 1:
 * 
 * 
 * Given nums = [1,1,2],
 * 
 * Your function should return length = 2, with the first two elements of nums
 * being 1 and 2 respectively.
 * 
 * It doesn't matter what you leave beyond the returned length.
 * 
 * Example 2:
 * 
 * 
 * Given nums = [0,0,1,1,1,2,2,3,3,4],
 * 
 * Your function should return length = 5, with the first five elements of nums
 * being modified to 0, 1, 2, 3, and 4 respectively.
 * 
 * It doesn't matter what values are set beyond the returned length.
 * 
 * 
 * Clarification:
 * 
 * Confused why the returned value is an integer but your answer is an array?
 * 
 * Note that the input array is passed in by reference, which means
 * modification to the input array will be known to the caller as well.
 * 
 * Internally you can think of this:
 * 
 * 
 * // nums is passed in by reference. (i.e., without making a copy)
 * int len = removeDuplicates(nums);
 * 
 * // any modification to nums in your function would be known by the caller.
 * // using the length returned by your function, it prints the first len
 * elements.
 * for (int i = 0; i < len; i++) {
 * print(nums[i]);
 * }
 * 
 */

impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        // 最佳实践: nums.dedup();  竟然变慢了 0ms -> 4ms
        if nums.len() < 2 {
            return nums.len() as i32
        }
        let mut i = 0;
        for j in 1..nums.len(){
            if nums[j] != nums[i]{
                i += 1;
                nums[i] = nums[j];
            }
        }
        1 + i as i32
    }
}

pub struct Solution;

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn it_works() {
        let mut v = vec![];
        assert_eq!(Solution::remove_duplicates(&mut v), 0);
        assert_eq!(&v[..], []);

        let mut v = vec![10];
        assert_eq!(Solution::remove_duplicates(&mut v), 1);
        assert_eq!(&v[0..1], [10]);

        let mut v = vec![1, 1, 2];
        assert_eq!(Solution::remove_duplicates(&mut v), 2);
        assert_eq!(&v[0..2], [1, 2]);

        let mut v = vec![0, 0, 1, 1, 1, 2, 2, 3, 3, 4];
        assert_eq!(Solution::remove_duplicates(&mut v), 5);
        assert_eq!(&v[0..5], [0, 1, 2, 3, 4]);
    }
}