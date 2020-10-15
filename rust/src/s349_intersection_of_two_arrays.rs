/*
 * @lc app=leetcode.cn id=349 lang=rust
 *
 * [349] 两个数组的交集
 */

// @lc code=start
use std::collections::HashSet;
impl Solution {
    
    pub fn intersection(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        let mut iset = HashSet::new();
        let mut uset = HashSet::new();

        for i in nums1 {
            uset.insert(i);
        }
        for i in nums2 {
            if uset.contains(&i) {
                iset.insert(i);
            }
        }
        iset.into_iter().collect()
    }

    pub fn intersection_sort(mut nums1: Vec<i32>, mut nums2: Vec<i32>) -> Vec<i32> {
        let mut nums = Vec::new();
        nums1.sort();
        nums2.sort();
        let mut i = 0;
        let mut j = 0;
        while i < nums1.len() && j < nums2.len() {
            if nums1[i] > nums2[j] { j += 1; continue; }
            if nums1[i] == nums2[j] { nums.push(nums1[i]); i += 1; j += 1; continue; }
            if nums1[i] < nums2[j] { i += 1; continue; }
        }
        nums.dedup();
        nums
    }
}
// @lc code=end


pub struct Solution;

#[cfg(test)]

mod test {
    use super::Solution;
    #[test]
    fn it_works() {
        assert_eq!(Solution::intersection(vec![1,2,2,1], vec![2,2]), vec![2]);
        assert_eq!(Solution::intersection_sort(vec![1,2,2,1], vec![2,2]), vec![2]);
    }
}